import type { InitialOptionsTsJest } from 'ts-jest';

const config: InitialOptionsTsJest = {
  preset: 'ts-jest/presets/default-esm', // or other ESM presets
  testMatch: ['**/tests/**/*.ts'],
  globals: {
    'ts-jest': {
      useESM: true,
    },
  },
  moduleNameMapper: {
    '^(\\.{1,2}/.*)\\.js$': '$1',
  },
  modulePathIgnorePatterns: ['vendor/', 'node_modules/', 'tests/helpers'],
};

export default config;
