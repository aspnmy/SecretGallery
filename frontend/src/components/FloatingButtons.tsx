'use client';

export default function FloatingButtons() {
  return (
    <div className="floating-buttons">
      <a href="/" className="floating-btn home-btn">
        🏠
      </a>
      <button onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })} className="floating-btn top-btn">
        ↑
      </button>
      <button onClick={() => window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' })} className="floating-btn bottom-btn">
        ↓
      </button>
    </div>
  );
}