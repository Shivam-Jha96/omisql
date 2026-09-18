import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Get the executable path from configuration or fallback to 'omnisql'
  const config = workspace.getConfiguration('omnisql');
  const executablePath = config.get<string>('executablePath', 'omnisql');

  const runOptions: Executable = {
    command: executablePath,
    args: ['lsp'], // Run the Language Server Protocol command
    options: {
      env: {
        ...process.env,
        RUST_LOG: 'info'
      }
    }
  };

  const serverOptions: ServerOptions = {
    run: runOptions,
    debug: runOptions
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for SQL documents
    documentSelector: [{ scheme: 'file', language: 'sql' }],
    synchronize: {
      // Notify the server about file changes to '.clientrc files contained in the workspace
      fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
    }
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    'omnisqlServer',
    'OmniSQL Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
