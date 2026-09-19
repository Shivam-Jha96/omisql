"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    // Get the executable path from configuration or fallback to 'omnisql'
    const config = vscode_1.workspace.getConfiguration('omnisql');
    const executablePath = config.get('executablePath', 'omnisql');
    const schemaPath = config.get('schemaPath', '');
    let args = ['lsp'];
    if (schemaPath && schemaPath.trim() !== '') {
        args.push('--schema', schemaPath);
    }
    const runOptions = {
        command: executablePath,
        args: args, // Run the Language Server Protocol command with args
        options: {
            env: {
                ...process.env,
                RUST_LOG: 'info'
            }
        }
    };
    const serverOptions = {
        run: runOptions,
        debug: runOptions
    };
    // Options to control the language client
    const clientOptions = {
        // Register the server for SQL documents
        documentSelector: [{ scheme: 'file', language: 'sql' }],
        synchronize: {
            // Notify the server about file changes to '.clientrc files contained in the workspace
            fileEvents: vscode_1.workspace.createFileSystemWatcher('**/.clientrc')
        }
    };
    // Create the language client and start the client.
    client = new node_1.LanguageClient('omnisqlServer', 'OmniSQL Language Server', serverOptions, clientOptions);
    // Start the client. This will also launch the server
    client.start();
}
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
//# sourceMappingURL=extension.js.map