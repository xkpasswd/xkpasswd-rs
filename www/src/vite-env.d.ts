/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_GIT_SHA?: string;
  // more env variables...
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
