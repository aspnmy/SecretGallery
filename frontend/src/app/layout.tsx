import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import '@/styles/globals.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: {
    default: 'GoComicMosaic',
    template: '%s - GoComicMosaic',
  },
  description: '开源影视资源共建平台',
  keywords: ['影视资源', '漫画', '共建平台', '开源'],
  authors: [{ name: 'GoComicMosaic Team' }],
  creator: 'GoComicMosaic Team',
  publisher: 'GoComicMosaic Team',
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
    <html lang="zh-CN">
      <body className={inter.className}>
        <div className="min-h-screen bg-background text-foreground">
          <header className="sticky top-0 z-50 bg-background/80 backdrop-blur-md border-b">
            <div className="container mx-auto px-4 py-3 flex justify-between items-center">
              <h1 className="text-xl font-bold">GoComicMosaic</h1>
              <nav>
                <ul className="flex space-x-4">
                  <li><a href="/resources" className="hover:text-primary transition-colors">资源列表</a></li>
                  <li><a href="/submit" className="hover:text-primary transition-colors">提交资源</a></li>
                  <li><a href="/login" className="hover:text-primary transition-colors">登录</a></li>
                  <li><a href="/admin" className="hover:text-primary transition-colors">管理</a></li>
                </ul>
              </nav>
            </div>
          </header>
          <main className="container mx-auto px-4 py-6">
            {children}
          </main>
          <footer className="mt-auto py-6 border-t">
            <div className="container mx-auto px-4 text-center text-sm text-muted">
              <p>© 2025 GoComicMosaic. 开源影视资源共建平台.</p>
            </div>
          </footer>
        </div>
      </body>
    </html>
  );
}