import './styles.scss';

function setup() {
  const form = document.getElementById('text-form') as HTMLFormElement | null;
  const card = document.getElementById('last-card') as HTMLElement | null;
  if (!form || !card) return;

  form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const fd = new FormData(form);
    await fetch('/submit', { method: 'POST', body: new URLSearchParams(fd as any) as any });
  });

  const ev = new EventSource('/events');
  ev.onmessage = (evt) => {
    const text = (evt.data || '').trim();
    if (text) {
      card.textContent = text;
    } else {
      card.innerHTML = '<em>No text submitted yet.</em>';
    }
  };
}

document.addEventListener('DOMContentLoaded', setup);
