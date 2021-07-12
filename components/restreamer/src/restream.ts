import App from './App.svelte';
import Empty from './Empty.svelte';

const app = new App({
  target: document.body,
  props: {
    mainComponent: Empty,
    toolbarComponent: Empty,
    apiUrl: '/api-out',
  },
});

(window as any).app = app;
export default app;