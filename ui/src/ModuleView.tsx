import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { SecurityBadge } from './SecurityBadge';
import { renderMarkdown } from './utils/markdown';

export interface ModuleItem {
  id: string;
  source: string;
  destination: string;
  title: string;
  content: string;
  created_at: number;
  is_valid_bpsec: boolean;
}

export function ModuleView() {
  const [items, setItems] = useState<ModuleItem[]>([]);
  const [title, setTitle] = useState('');
  const [destination, setDestination] = useState('');
  const [content, setContent] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchItems = async () => {
    try {
      const res = await fetch('/api/template_mod/v1/items');
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setItems(data);
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    fetchItems();
  }, []);

  const handleCreate = async (e: Event) => {
    e.preventDefault();
    if (!destination.trim() || !title.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/template_mod/v1/items', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ destination, title, content }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setTitle('');
      setContent('');
      await fetchItems();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="max-w-4xl mx-auto p-4 space-y-6">
      <header class="border-b pb-4 dark:border-slate-700 flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-slate-900 dark:text-white">Template DTN Extension</h1>
          <p class="text-sm text-slate-500">Exemple de module fullstack DTN Postmaster</p>
        </div>
        <SecurityBadge status="valid" />
      </header>

      {error && (
        <div class="p-3 bg-rose-50 text-rose-700 border border-rose-200 rounded-lg text-sm">
          {error}
        </div>
      )}

      <form onSubmit={handleCreate} class="space-y-4 bg-slate-50 dark:bg-slate-800/50 p-4 rounded-xl border border-slate-200 dark:border-slate-700">
        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-100">Créer un nouvel élément</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-medium text-slate-600 dark:text-slate-300 mb-1">Destination EID</label>
            <input
              type="text"
              placeholder="dtn://node-b/template"
              value={destination}
              onInput={(e) => setDestination((e.target as HTMLInputElement).value)}
              required
              class="w-full px-3 py-2 border rounded-lg dark:bg-slate-800 dark:border-slate-600 dark:text-white text-sm"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-slate-600 dark:text-slate-300 mb-1">Titre</label>
            <input
              type="text"
              placeholder="Sujet de l'échange"
              value={title}
              onInput={(e) => setTitle((e.target as HTMLInputElement).value)}
              required
              class="w-full px-3 py-2 border rounded-lg dark:bg-slate-800 dark:border-slate-600 dark:text-white text-sm"
            />
          </div>
        </div>
        <div>
          <label class="block text-xs font-medium text-slate-600 dark:text-slate-300 mb-1">Contenu (Markdown supporté)</label>
          <textarea
            placeholder="Texte du message ou de la commande..."
            value={content}
            onInput={(e) => setContent((e.target as HTMLTextAreaElement).value)}
            rows={3}
            class="w-full px-3 py-2 border rounded-lg dark:bg-slate-800 dark:border-slate-600 dark:text-white text-sm"
          />
        </div>
        <button
          type="submit"
          disabled={loading}
          class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-sm font-medium transition disabled:opacity-50"
        >
          {loading ? 'Envoi...' : 'Envoyer via BPA'}
        </button>
      </form>

      <div class="space-y-4">
        <h2 class="text-lg font-semibold text-slate-800 dark:text-slate-100">Éléments enregistrés ({items.length})</h2>
        {items.length === 0 ? (
          <p class="text-sm text-slate-500 italic">Aucun élément enregistré pour le moment.</p>
        ) : (
          <div class="space-y-3">
            {items.map((item) => (
              <div key={item.id} class="p-4 bg-white dark:bg-slate-800 rounded-xl border border-slate-200 dark:border-slate-700 space-y-2">
                <div class="flex justify-between items-start">
                  <h3 class="font-bold text-slate-900 dark:text-white">{item.title}</h3>
                  <SecurityBadge status={item.is_valid_bpsec ? 'valid' : 'unsigned'} />
                </div>
                <div class="text-xs text-slate-500 flex space-x-4">
                  <span>De: <code>{item.source}</code></span>
                  <span>Vers: <code>{item.destination}</code></span>
                </div>
                <div
                  class="prose prose-sm dark:prose-invert max-w-none text-slate-700 dark:text-slate-300"
                  dangerouslySetInnerHTML={{ __html: renderMarkdown(item.content) }}
                />
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
