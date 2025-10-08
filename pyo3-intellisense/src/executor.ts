import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import * as toml from 'toml';
import { execFile } from 'child_process';

export function isPyo3Project(): boolean {
    const folders = vscode.workspace.workspaceFolders;
    if (!folders) {return false;}
    for (const folder of folders) {
        const cargoTomlPath = path.join(folder.uri.fsPath, 'Cargo.toml');
        if (!fs.existsSync(cargoTomlPath)) {continue;}
        const content = fs.readFileSync(cargoTomlPath, 'utf-8');
        try {
            const parsed = toml.parse(content);
            const deps = { ...parsed.dependencies, ...parsed['dev-dependencies'] };
            if (deps && (deps.pyo3 !== undefined || deps.maturin !== undefined)) {
                return true;
            }
        } catch (e) {
            console.log(e);
            return false;
        }
    }
    return false;
}

export function getCliBinaryPath(context: vscode.ExtensionContext): string {
    return path.join(context.extensionPath, 'bin', 'pryo3-stubs-cli-macos');
}

export function runCli(context: vscode.ExtensionContext, libPath: string) {
    const cliPath = getCliBinaryPath(context);
    const outputPath = path.join(path.dirname(libPath), 'stubs.pyi');
    execFile(cliPath, ['--input', libPath, '--output', outputPath], (error, stdout, stderr) => {
        if (error) {
            vscode.window.showErrorMessage(`Stub generation failed: ${stderr || error.message}`);
        } else {
            vscode.window.showInformationMessage('Stub generation succeeded!');
        }
    });
}