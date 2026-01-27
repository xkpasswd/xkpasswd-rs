import js from '@eslint/js';
import tsParser from '@typescript-eslint/parser';
import preact from 'eslint-config-preact';
import prettier from 'eslint-config-prettier';

export default [
    js.configs.recommended,
    ...preact,
    prettier,
    {
        files: ['**/*.{ts,tsx}'],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                ecmaFeatures: { jsx: true },
                ecmaVersion: 'latest',
                sourceType: 'module',
            },
        },
        rules: {
            'no-unused-vars': ['error', { args: 'none' }],
        },
    },
    {
        ignores: ['**/*.d.ts', 'dist/', 'xkpasswd/', 'node_modules/'],
    },
];
