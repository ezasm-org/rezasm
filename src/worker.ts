import registerWebworker from "webworker-promise/lib/register";

type WasmExports = Omit<typeof import("../wasm/rezasm_wasm.js"), "default">;

// https://stackoverflow.com/questions/53501721/typescript-exclude-property-key-when-starts-with-target-string
// eslint-disable-next-line no-unused-vars,@typescript-eslint/no-unused-vars
type FilterStartsWith<Set, Needle extends string> = Set extends `${Needle}${infer _}` ? Set : never
// https://stackoverflow.com/questions/75418099/shorter-keys-with-typescript-key-remapping-removing-prefix
type RemovePrefix<K extends PropertyKey, Prefix extends string> =
    K extends `${Prefix}${infer P}` ? P : K

type NonWasmCommandStrings = "status" | "ping"
export type ValidWasmCommandStrings = RemovePrefix<FilterStartsWith<keyof WasmExports, "wasm_">, "wasm_">;
type ValidCommandStrings = ValidWasmCommandStrings | NonWasmCommandStrings;
type WasmFunctions = Pick<WasmExports, `wasm_${ValidWasmCommandStrings}`>

let wasmFunctions: WasmFunctions;

export interface Message {
    command: ValidCommandStrings
    argument?: Record<string, unknown>
    shape?: string[]
}

if (self.__WASM_LOADED__ === undefined && self.document === undefined) {
    self.__WASM_LOADED__ = false;
    import("../wasm/rezasm_wasm.js").then(async (module) => {
        await module.default();
        wasmFunctions = Object.entries(module)
            .filter(([x,]) => x.startsWith("wasm_"))
            .reduce((left, [x, y]) => {
                left[x] = y;
                return left;
            }, {} as Record<string, unknown>) as WasmFunctions;
        self.__WASM_LOADED__ = true;
        console.log("WebAssembly code loaded");
    }).catch((error) => {
        console.log("WebAssembly could not load", error);
    });

    const worker = registerWebworker(async (message: Message) => {
        const command = message.command;
        if (command === "status") {
            return self.__WASM_LOADED__;
        }
        const data = message.argument ?? {};
        const shape = message.shape ?? [];
        const functionArguments: unknown[] = shape.map(arg => {
            const argument = data[arg];
            if (argument !== undefined) {
                return argument;
            } else {
                throw new Error(`Function '${command}' passed without required argument ${arg}`);
            }
        });

        if (command === "ping") {
            return "pong";
        }

        const wasmFunction = wasmFunctions[`wasm_${command}`];

        if (wasmFunction) {
            try {
                // @ts-expect-error -- There is no easy way to ensure this is correctly typed here.
                // We have this typed correctly in rust_functions.ts.
                return wasmFunction(...functionArguments);
            } catch (error) {
                throw new Error(String(error));
            }
        } else {
            throw new Error(`Invalid command: '${command}'`);
        }
    });

    self.emitPrintString = (string: string) => {
        worker.emit("wasm_print", string);
    };
}
