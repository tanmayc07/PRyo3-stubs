import * as vscode from 'vscode';
import { isPyo3Project } from './executor';

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "pyo3-intellisense" is now active!');

	let disposable = vscode.commands.registerCommand('pyo3-stubs.runGenerator', () => {
		vscode.window.showInformationMessage('Pyo3 Stubs Generator command executed!');

		if (isPyo3Project()) {
			const watcher = vscode.workspace.createFileSystemWatcher('**/src/lib.rs');
			watcher.onDidChange(uri => {
				vscode.window.showInformationMessage(`Rust file changed: ${uri.fsPath}`);
			});
		} else {
			vscode.window.showErrorMessage('Not a Pyo3/maturin project.');
		}
	});

	context.subscriptions.push(disposable);
}

export function deactivate() {}
