import { render } from 'preact';
import { ModuleView } from './ModuleView';

export { ModuleView };

const root = document.getElementById('app');
if (root) {
  render(<ModuleView />, root);
}
