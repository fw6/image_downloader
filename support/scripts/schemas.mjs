// @ts-check
import fs from "node:fs";
import {
    FetchingJSONSchemaStore,
    InputData,
    JSONSchemaInput,
    quicktype,
} from "quicktype-core";

async function main() {
    const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());
    const inputData = new InputData();
    inputData.addInput(schemaInput);
    inputData.addSource(
        "schema",
        {
            name: "SchemaTypes",
            uris: ["support/schemas/schema_types/SchemaTypes.json#/definitions/"],
        },
        () => new JSONSchemaInput(new FetchingJSONSchemaStore()),
    );

    const ts = await quicktype({
        inputData,
        lang: "typescript",
        rendererOptions: {},
    });
    console.log(ts.lines.join("\n"));

    // writeToFile("./languages/js/sdk-client/src/schemas.ts", ts.lines);
    // writeToFile("./crates/bitwarden-napi/src-ts/bitwarden_client/schemas.ts", ts.lines);

    // const cpp = await quicktype({
    //     inputData,
    //     lang: "cpp",
    //     rendererOptions: {
    //         namespace: "Imagekit::Sdk",
    //         "include-location": "global-include",
    //     },
    // });

    // cpp.lines.forEach((line, idx) => {
    //     // Replace DOMAIN for URI_DOMAIN, because DOMAIN is an already defined macro
    //     cpp.lines[idx] = line.replace(/DOMAIN/g, "URI_DOMAIN");
    // });

    // writeToFile("./languages/cpp/include/schemas.hpp", cpp.lines);

    // const java = await quicktypeMultiFile({
    //     inputData,
    //     lang: "java",
    //     rendererOptions: {
    //         package: "com.imagekit.sdk.schema",
    //         "java-version": "8",
    //     },
    // });

    // const javaDir = "./languages/java/src/main/java/com/bitwarden/sdk/schema/";
    // if (!fs.existsSync(javaDir)) {
    //     fs.mkdirSync(javaDir);
    // }
    // java.forEach((file, path) => {
    //     writeToFile(javaDir + path, file.lines);
    // });
}

main();

/**
 * @param {string} filename
 * @param {string[]} lines
 */
function writeToFile(filename, lines) {
    const output = fs.createWriteStream(filename);
    lines.forEach((line) => {
        output.write(line + "\n");
    });
    output.close();
}
