import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "pyo3-intellisense" is now active!');

	const disposable = vscode.commands.registerCommand('pyo3-intellisense.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from Pyo3 IntelliSense!');
	});

	context.subscriptions.push(disposable);
}

export function deactivate() {}
