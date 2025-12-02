'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { loginUser } from '@/services/authService';

interface LoginPageProps {
  searchParams: { [key: string]: string | string[] | undefined };
}

export default function LoginPage({ searchParams }: LoginPageProps) {
  const router = useRouter();
  const [formData, setFormData] = useState({
    username: '',
    password: '',
  });
  
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * 处理表单提交
   * @param e - 表单提交事件
   */
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!formData.username || !formData.password) {
      setError('请输入用户名和密码');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      
      // 登录成功后重定向到首页或管理页面
      await loginUser(formData);
      
      const redirectTo = typeof searchParams.redirect === 'string' ? searchParams.redirect : '/';
      router.push(redirectTo);
    } catch (err) {
      console.error('登录失败:', err);
      setError('用户名或密码错误，请重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-[calc(100vh-150px)]">
      <div className="w-full max-w-md card p-8 animate-fade-in-up">
        <h1 className="text-3xl font-bold text-center mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">登录</h1>
        
        {error && (
          <div className="mb-6 p-4 bg-danger/10 text-danger rounded-lg text-sm">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="username" className="form-label">用户名</label>
            <input
              type="text"
              id="username"
              value={formData.username}
              onChange={(e) => setFormData(prev => ({ ...prev, username: e.target.value }))}
              className="form-control"
              placeholder="请输入用户名"
              autoComplete="username"
            />
          </div>
          
          <div>
            <label htmlFor="password" className="form-label">密码</label>
            <input
              type="password"
              id="password"
              value={formData.password}
              onChange={(e) => setFormData(prev => ({ ...prev, password: e.target.value }))}
              className="form-control"
              placeholder="请输入密码"
              autoComplete="current-password"
            />
          </div>
          
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <input
                type="checkbox"
                id="remember"
                className="h-4 w-4 text-primary focus:ring-primary border-border rounded"
              />
              <label htmlFor="remember" className="ml-2 block text-sm text-gray">
                记住我
              </label>
            </div>
            
            <a href="/forgot-password" className="text-sm text-primary hover:underline transition-colors">
              忘记密码？
            </a>
          </div>
          
          <button
            type="submit"
            disabled={loading}
            className="w-full btn btn-primary"
          >
            {loading ? (
              <>
                <div className="loading mr-2"></div>
                <span className="btn-text">登录中...</span>
              </>
            ) : (
              <span className="btn-text">登录</span>
            )}
          </button>
        </form>
        
        <div className="mt-6 text-center text-sm">
          <p className="text-gray">
            还没有账号？{' '}
            <a href="/register" className="text-primary hover:underline transition-colors">
              立即注册
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}
