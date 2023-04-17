import type { ApiEndpoint } from './types';

export const tParseMdx: ApiEndpoint = {
  path: 'utils/parse_mdx',
  methods: ['POST'],
  description: 'Parse a mdx file decoded in base64',
  input: '{\n  test: string\n}',
  output: '{\n  test: string\n}'
};
