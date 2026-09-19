import * as fs from 'fs';

// OmniSQL passes the JSON-serialized AST via standard input.
const input = fs.readFileSync(0, 'utf-8');
const ast = JSON.parse(input);

const issues: any[] = [];

// Example Rule: "No Select Star"
// We iterate through the AST statements looking for a SelectStatement with an Asterisk column.
for (const statement of ast.statements) {
    if (statement.type === 'SelectStatement') {
        const hasAsterisk = statement.columns.some((col: any) => col.name === '*');
        if (hasAsterisk) {
            issues.push({
                rule: "NoSelectStar",
                level: "Warning",
                message: "Avoid using SELECT *. Explicitly declare the columns you need for better performance and contract stability.",
                line: statement.line || 1
            });
        }
    }
}

// OmniSQL expects a JSON array of LintIssue objects on standard output.
console.log(JSON.stringify(issues));
