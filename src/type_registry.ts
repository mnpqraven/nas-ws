import type { ApiEndpoint } from '$lib/server/types';

const tParseMdx: ApiEndpoint = {
  path: 'utils/parse_mdx',
  methods: ['POST'],
  description: 'Parse a mdx file decoded in base64',
  input: ['test: string'],
  output: ['test: string']
};

export default { tParseMdx };
