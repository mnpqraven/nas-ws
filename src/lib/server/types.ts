// TODO: find a way to auto generate this using rust runnables
export interface ApiEndpoint {
  path: string;
  methods: ('GET' | 'POST' | 'UPDATE' | 'DELETE')[];
  description: string;
  input: string | string[];
  output: string | string[];
}
