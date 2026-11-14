// Sanitized markdown and HTML utility for template-dtnpm-mod

export function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

export function escapeAttribute(attr: string): string {
  return attr
    .replace(/&/g, '&amp;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');
}

export function isSafeImageUrl(url: string, allowExternalImages: boolean = false): boolean {
  const trimmed = url.trim();
  if (!trimmed || /[\x00-\x1f\x7f]/.test(trimmed)) {
    return false;
  }
  const isRasterDataUri = /^data:image\/(?:png|jpeg|jpg|gif|webp);base64,[A-Za-z0-9+/=\s]+$/i.test(trimmed);
  if (isRasterDataUri) {
    return true;
  }
  if (!trimmed.startsWith('//') && !/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(trimmed)) {
    return /^[a-zA-Z0-9\-._~/?#=&%]+$/i.test(trimmed);
  }
  if (allowExternalImages) {
    return /^https?:\/\/[a-zA-Z0-9\-._~:/?#[\]@!$&'()*+,;=]+$/i.test(trimmed);
  }
  return false;
}

export function isSafeLinkUrl(url: string): boolean {
  const trimmed = url.trim();
  if (!trimmed || /[\x00-\x1f\x7f]/.test(trimmed) || /&[#a-zA-Z0-9]+;/.test(trimmed)) {
    return false;
  }
  const schemeMatch = trimmed.match(/^([a-zA-Z][a-zA-Z0-9+.-]*):/);
  if (schemeMatch) {
    const scheme = schemeMatch[1].toLowerCase();
    return ['http', 'https', 'dtn', 'mailto', 'tel'].includes(scheme);
  }
  if (trimmed.startsWith('//')) {
    return /^\/\/[a-zA-Z0-9\-._~:/?#[\]@!$&'()*+,;=]+$/i.test(trimmed);
  }
  return /^[a-zA-Z0-9\-._~:/?#[\]@!$&'()*+,;=]+$/i.test(trimmed);
}

function generateNonce(): string {
  if (typeof globalThis.crypto !== 'undefined' && typeof globalThis.crypto.getRandomValues === 'function') {
    const buf = new Uint32Array(2);
    globalThis.crypto.getRandomValues(buf);
    return buf[0].toString(36) + buf[1].toString(36);
  }
  return Math.random().toString(36).slice(2) + Math.random().toString(36).slice(2);
}

export function renderMarkdown(
  content: string,
  options: { allowImages?: boolean; allowExternalImages?: boolean } = {}
): string {
  const placeholders: { token: string; html: string }[] = [];
  const nonce = generateNonce();
  let tokenCounter = 0;

  function createPlaceholder(html: string): string {
    const token = `\x00DTNMD${nonce}X${tokenCounter++}\x00`;
    placeholders.push({ token, html });
    return token;
  }

  let textToProcess = content.replace(/\x00/g, '');

  textToProcess = textToProcess.replace(/```([a-zA-Z0-9_-]*)\n?([\s\S]*?)```/g, (_match, _lang, code) => {
    return createPlaceholder(
      `<pre class="my-2 p-3 bg-slate-900 text-slate-100 rounded-xl text-xs font-mono overflow-x-auto whitespace-pre-wrap">${escapeHtml(code.trim())}</pre>`
    );
  });

  textToProcess = textToProcess.replace(/`([^`]+)`/g, (_match, code) => {
    return createPlaceholder(
      `<code class="px-1.5 py-0.5 bg-slate-200 dark:bg-slate-700 rounded text-xs font-mono">${escapeHtml(code)}</code>`
    );
  });

  if (options.allowImages) {
    textToProcess = textToProcess.replace(/!\[([^\]]*)\]\(([^)\s]+(?:\s+[^)]*)?)\)/g, (_match, alt, rawUrl) => {
      const cleanUrl = rawUrl.trim();
      if (!isSafeImageUrl(cleanUrl, Boolean(options.allowExternalImages))) {
        return `[Image blocked: unsafe URL]`;
      }
      const safeAlt = escapeHtml(alt || 'Image');
      const safeUrl = escapeAttribute(cleanUrl);
      const tag = `<div class="my-3"><img src="${safeUrl}" alt="${safeAlt}" class="max-h-96 rounded-xl shadow border border-slate-200 dark:border-slate-700 object-contain mx-auto" /><p class="text-center text-xs text-slate-500 dark:text-slate-400 mt-1">${safeAlt}</p></div>`;
      return createPlaceholder(tag);
    });
  }

  const linkRegex = /\[([^\]]+)\]\(\s*([^)\s]+)(?:\s+"([^"]{0,200})"|\s+'([^']{0,200})')?\s*\)/g;
  textToProcess = textToProcess.replace(linkRegex, (_match, label, url, dqTitle, sqTitle) => {
    const cleanUrl = url.trim();
    if (!isSafeLinkUrl(cleanUrl)) {
      return `[${escapeHtml(label)}](unsafe link blocked)`;
    }
    const safeUrl = escapeAttribute(cleanUrl);
    const title = dqTitle ?? sqTitle;
    const titleAttr = title ? ` title="${escapeAttribute(title)}"` : '';
    let formattedLabel = escapeHtml(label);
    formattedLabel = formattedLabel.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
    formattedLabel = formattedLabel.replace(/\*([^*]+)\*/g, '<em>$1</em>');
    formattedLabel = formattedLabel.replace(/_([^_]+)_/g, '<em>$1</em>');
    const tag = `<a href="${safeUrl}" target="_blank" rel="noopener noreferrer"${titleAttr} class="text-emerald-600 dark:text-emerald-400 underline hover:opacity-85">${formattedLabel}</a>`;
    return createPlaceholder(tag);
  });

  let formatted = escapeHtml(textToProcess);
  formatted = formatted.replace(/^### (.*$)/gm, '<h3 class="text-base font-bold text-slate-900 dark:text-white mt-3 mb-1">$1</h3>');
  formatted = formatted.replace(/^## (.*$)/gm, '<h2 class="text-lg font-bold text-emerald-600 dark:text-emerald-400 mt-4 mb-2">$1</h2>');
  formatted = formatted.replace(/^# (.*$)/gm, '<h1 class="text-xl font-extrabold text-slate-900 dark:text-white mt-4 mb-2 pb-1 border-b border-slate-200 dark:border-slate-700">$1</h1>');
  formatted = formatted.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
  formatted = formatted.replace(/\*([^*]+)\*/g, '<em>$1</em>');
  formatted = formatted.replace(/_([^_]+)_/g, '<em>$1</em>');
  formatted = formatted.replace(/^[-*+] (.*$)/gm, '<li class="ml-4 list-disc">$1</li>');
  formatted = formatted.replace(/(<\/(?:li|pre|h[1-3])>)\n/g, '$1');
  formatted = formatted.replace(/\n/g, '<br/>');

  for (let i = placeholders.length - 1; i >= 0; i--) {
    const item = placeholders[i];
    formatted = formatted.replaceAll(item.token, item.html);
  }

  return formatted;
}
