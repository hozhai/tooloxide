export interface HelloAPI {
  message: string;
}

export interface SearchAPIElem {
  id: string;
  name: string;
  description: string;
}

export interface SearchAPI {
  headers: {
    userAgent: string;
  };
  path: string;
  query: {
    results: SearchAPIElem[];
    term: string;
  };
}
