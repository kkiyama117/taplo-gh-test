// @ts-ignore
import loadTaplo from "../../../taplo/Cargo.toml";

/**
 * Taplo formatter options. (https://taplo.tamasfe.dev/configuration/#formatting-options)
 */
export interface FormatterOptions {
  /**
   * Align consecutive entries vertically.
   */
  alignEntries?: boolean;

  /**
   * Append trailing commas for multi-line arrays.
   */
  arrayTrailingComma?: boolean;

  /**
   * Expand arrays to multiple lines that exceed the maximum column width.
   */
  arrayAutoExpand?: boolean;

  /**
   * Collapse arrays that don't exceed the maximum column width and don't contain comments.
   */
  arrayAutoCollapse?: boolean;

  /**
   * Omit white space padding from single-line arrays
   */
  compactArrays?: boolean;

  /**
   * Omit white space padding from the start and end of inline tables.
   */
  compactInlineTables?: boolean;

  /**
   * Maximum column width in characters, affects array expansion and collapse, this doesn't take whitespace into account.
   * Note that this is not set in stone, and works on a best-effort basis.
   */
  columnWidth?: number;

  /**
   * Indent based on tables and arrays of tables and their subtables, subtables out of order are not indented.
   */
  indentTables?: boolean;

  /**
   * The substring that is used for indentation, should be tabs or spaces (but technically can be anything).
   */
  indentString?: string;

  /**
   * Add trailing newline at the end of the file if not present.
   */
  trailingNewline?: boolean;

  /**
   * Alphabetically reorder keys that are not separated by empty lines.
   */
  reorderKeys?: boolean;

  /**
   * Maximum amount of allowed consecutive blank lines. This does not affect the whitespace at the end of the document, as it is always stripped.
   */
  allowedBlankLines?: number;

  /**
   * Use CRLF for line endings.
   */
  crlf?: boolean;
}

/**
 * Options for the format function.
 */
export interface FormatOptions {
  /**
   * Ignore syntax errors, and format anyway.
   *
   * Setting this can be potentially destructive,
   * if the TOML document is invalid.
   */
  ignoreErrors?: boolean;
  /**
   * Options to pass to the formatter. (https://taplo.tamasfe.dev/configuration/#formatting-options)
   */
  options?: FormatterOptions;
}

/**
 * Byte range within a TOML document.
 */
export interface Range {
  /**
   * Start byte index.
   */
  start: number;
  /**
   * Exclusive end index.
   */
  end: number;
}

/**
 * An lint error.
 */
export interface LintError {
  /**
   * A range within the TOML document if any.
   */
  range?: Range;

  /**
   * The error message.
   */
  error: string;
}

/**
 * The object returned from the lint function.
 */
export interface LintResult {
  /**
   * Lint errors, if any.
   *
   * This includes syntax, semantic and schema errors as well.
   */
  errors: Array<LintError>;
}

/**
 * Options for the lint function.
 */
export interface LintOptions {
  /**
   * Optional JSON schema for validation, can be a JSON string or an object.
   */
  schema?: string | any;

  /**
   * By default validation errors based on JSON Schema return a human-readable
   * path to the invalid part in the error message.
   *
   * If this property is `true`, the byte range will be returned instead in the error,
   * just like for syntax errors.
   */
  schemaRanges?: boolean;
}

/**
 * This class allows for usage of the library in a synchronous context
 * after being asynchronously initialized once.
 *
 * It cannot be constructed with `new`, and instead must be
 * created by calling `initialize`.
 *
 * Example usage:
 *
 * ```js
 * import { Taplo } from "taplo";
 *
 * // Somewhere at the start of your app.
 * const taplo = await Taplo.initialize();
 * // ...
 * // The other methods will not return promises.
 * const formatted = taplo.format(tomlDocument);
 * ```
 */
export class Taplo {
  private static taplo: any | undefined;
  private static initializing: boolean = false;

  private constructor() {
    if (!Taplo.initializing) {
      throw new Error(
        `an instance of Taplo can only be created by calling the "initialize" static method`
      );
    }
  }

  public static async initialize() {
    if (typeof Taplo.taplo === "undefined") {
      Taplo.taplo = await loadTaplo();
    }

    Taplo.initializing = true;
    const t = new Taplo();
    Taplo.initializing = false;

    return t;
  }

  /**
   * Lint a TOML document, this function returns
   * both syntax and semantic (e.g. conflicting keys) errors.
   *
   * If a JSON schema is given, the TOML document will be validated with it
   * only if it is syntactically valid.
   *
   * @throws If the given JSON schema is invalid.
   *
   * @param toml TOML document.
   * @param options Optional additional options.
   */
  public lint(toml: string, options?: LintOptions): LintResult {
    let schema = options?.schema;

    if (typeof schema !== "undefined") {
      if (typeof schema !== "string") {
        schema = JSON.stringify(schema);
      }
    }

    try {
      return Taplo.taplo.lint(toml, !!(options?.schemaRanges ?? true), schema);
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Format the given TOML document.
   *
   * @throws Throws if the document contains syntax errors and the `ignoreErrors` option is false.
   *
   * @param toml TOML document.
   * @param options Optional format options.
   */
  public format(toml: string, options?: FormatOptions): string {
    let optsJson = undefined;

    if (typeof options?.options !== "undefined") {
      optsJson = JSON.stringify(options.options);
    }

    try {
      return Taplo.taplo.format(
        toml,
        !!(options?.ignoreErrors ?? false),
        optsJson
      );
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Encode the given JavaScript object to TOML.
   *
   * @throws If the given object cannot be serialized to TOML.
   *
   * @param data JSON compatible JavaScript object or JSON string.
   */
  public encode(data: object | string): string {
    if (typeof data !== "string") {
      data = JSON.stringify(data);
    }

    try {
      return Taplo.taplo.from_json(data);
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Decode the given TOML string to a JavaScript object.
   *
   * @throws If data is not valid TOML.
   *
   * @param {string} data TOML string.
   */
  public decode<T extends object = any>(data: string): T;

  /**
   * Convert the given TOML string to JSON.
   *
   * @throws If data is not valid TOML.
   *
   * @param data TOML string.
   * @param {boolean} [parse] Whether to parse the given JSON string.
   */
  public decode(data: string, parse: false): string;

  public decode<T extends object = any>(
    data: string,
    parse?: boolean
  ): T | string {
    let v: string;
    try {
      v = Taplo.taplo.to_json(data);
    } catch (e) {
      throw new Error(e);
    }

    if (parse) {
      return JSON.parse(v);
    } else {
      return v;
    }
  }
}
