import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import '@/styles/globals.css';
import FloatingButtons from '@/components/FloatingButtons';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: {
    default: '密影库',
    template: '%s - 密影库',
  },
  description: '私密图片和视频安全存储平台',
  keywords: ['私密图片', '私密视频', '安全存储', '加密'],
  authors: [{ name: '密影库团队' }],
  creator: '密影库团队',
  publisher: '密影库团队',
  formatDetection: {
    email: false,
    address: false,
    telephone: false,
  },
  metadataBase: new URL('http://localhost:3000'),
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN" className="scroll-smooth">
      <body className={`${inter.className} app-container`}>
        <header className="app-header">
          <div className="container">
            <div className="header-inner">
              <div className="brand">
                <a href="/" className="brand-link">
                  <span className="brand-icon">📸</span>
                  <span className="brand-text">密影库</span>
                </a>
              </div>
              <nav>
                <ul className="nav-links">
                  <li><a href="/resources" className="nav-link">资源列表</a></li>
                  <li><a href="/submit" className="nav-link">提交资源</a></li>
                  <li><a href="/login" className="nav-link">登录</a></li>
                  <li><a href="/admin" className="nav-link">管理</a></li>
                </ul>
              </nav>
              <div className="header-actions">
                <div className="button-group">
                  <a href="/submit" className="btn btn-primary">
                    <span className="btn-text">+ 提交资源</span>
                  </a>
                </div>
              </div>
            </div>
          </div>
        </header>
        <main className="main-content">
          <div className="container">
            <div className="content-container fade-in-up">
              {children}
            </div>
          </div>
        </main>
        <footer className="app-footer">
          <div className="container">
            <div className="footer-inner">
              <div className="footer-row">
                <a href="/resources" className="footer-link">资源列表</a>
                <a href="/submit" className="footer-link">提交资源</a>
                <a href="/login" className="footer-link">登录</a>
                <a href="/admin" className="footer-link">管理</a>
              </div>
              <div className="footer-divider"></div>
              <div className="copyright">
                <p>© 2025 密影库. 私密图片和视频安全存储平台.</p>
                <p>使用 Next.js + React 18 + TypeScript 构建</p>
              </div>
            </div>
          </div>
        </footer>
        {/* 悬浮按钮 */}
        <FloatingButtons />
      </body>
    </html>
  );
}