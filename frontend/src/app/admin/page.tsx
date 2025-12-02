'use client';

import { useState, useEffect } from 'react';
import { Resource } from '@/types';
import { getResources, deleteResource } from '@/services/resourceService';
import { isLoggedIn } from '@/services/authService';
import { useRouter } from 'next/navigation';

interface AdminPageProps {
  searchParams: { [key: string]: string | string[] | undefined };
}

export default function AdminPage({ searchParams }: AdminPageProps) {
  const router = useRouter();
  const [resources, setResources] = useState<Resource[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState<number | null>(null);

  // 检查用户是否已登录
  useEffect(() => {
    if (!isLoggedIn()) {
      router.push('/login?redirect=/admin');
    }
  }, [router]);

  // 获取资源列表
  useEffect(() => {
    const fetchResources = async () => {
      try {
        setLoading(true);
        const data = await getResources({ type: '', page: 1, limit: 100 });
        setResources(data);
      } catch (err) {
        console.error('获取资源列表失败:', err);
        setError('获取资源列表失败，请稍后重试');
      } finally {
        setLoading(false);
      }
    };

    fetchResources();
  }, []);

  /**
   * 处理资源删除
   * @param id - 要删除的资源ID
   */
  const handleDelete = async (id: number) => {
    try {
      setLoading(true);
      await deleteResource(id);
      setResources(prev => prev.filter(r => r.id !== id));
      setDeleteConfirm(null);
    } catch (err) {
        console.error('删除资源 ' + id + ' 失败:', err);
        setError('删除资源失败，请稍后重试');
      } finally {
        setLoading(false);
      }
  };

  if (loading && resources.length === 0) {
    return (
      <div className="flex justify-center items-center py-20">
        <div className="text-center">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary mb-4"></div>
          <p>加载中...</p>
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6 flex justify-between items-center">
        <h1 className="text-3xl font-bold">管理面板</h1>
        <a
          href="/submit"
          className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors flex items-center gap-1"
        >
          + 添加资源
        </a>
      </div>
      
      {error && (
        <div className="mb-4 p-4 bg-red-50 text-red-500 rounded-lg">
          {error}
        </div>
      )}

      <div className="bg-secondary rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">资源管理</h2>
        
        {resources.length === 0 ? (
          <div className="text-center py-12 text-muted">
            暂无资源
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full border-collapse">
              <thead>
                <tr className="border-b border-border">
                  <th className="text-left py-3 px-4 font-medium">ID</th>
                  <th className="text-left py-3 px-4 font-medium">标题</th>
                  <th className="text-left py-3 px-4 font-medium">类型</th>
                  <th className="text-left py-3 px-4 font-medium">作者</th>
                  <th className="text-left py-3 px-4 font-medium">来源</th>
                  <th className="text-left py-3 px-4 font-medium">创建时间</th>
                  <th className="text-left py-3 px-4 font-medium">操作</th>
                </tr>
              </thead>
              <tbody>
                {resources.map((resource) => (
                  <tr key={resource.id} className="border-b border-border hover:bg-background transition-colors">
                    <td className="py-3 px-4 text-sm">{resource.id}</td>
                    <td className="py-3 px-4 text-sm font-medium">
                      <a href={"/resources/" + resource.id} className="text-primary hover:underline">
                        {resource.title}
                      </a>
                    </td>
                    <td className="py-3 px-4 text-sm">
                      {resource.resource_type === 'video' ? (
                        <span className="px-2 py-1 rounded-full text-xs bg-blue-100 text-blue-800">视频</span>
                      ) : (
                        <span className="px-2 py-1 rounded-full text-xs bg-green-100 text-green-800">图片</span>
                      )}
                    </td>
                    <td className="py-3 px-4 text-sm">{resource.author || '-'}</td>
                    <td className="py-3 px-4 text-sm">{resource.source}</td>
                    <td className="py-3 px-4 text-sm">
                      {new Date(resource.created_at).toLocaleDateString()}
                    </td>
                    <td className="py-3 px-4 text-sm">
                      <div className="flex gap-2">
                        <a
                          href={"/admin/edit/" + resource.id}
                          className="px-3 py-1 bg-primary/10 text-primary rounded hover:bg-primary/20 transition-colors text-xs"
                        >
                          编辑
                        </a>
                        <button
                          onClick={() => setDeleteConfirm(resource.id)}
                          className="px-3 py-1 bg-red-100 text-red-800 rounded hover:bg-red-200 transition-colors text-xs"
                        >
                          删除
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* 删除确认对话框 */}
      {deleteConfirm !== null && (
        <div className="fixed inset-0 bg-black/50 flex justify-center items-center z-50">
          <div className="bg-secondary p-6 rounded-lg shadow-xl max-w-md w-full">
            <h3 className="text-lg font-semibold mb-3">确认删除</h3>
            <p className="mb-4">您确定要删除这个资源吗？此操作不可撤销。</p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setDeleteConfirm(null)}
                className="px-4 py-2 border border-border rounded-lg hover:bg-background transition-colors"
              >
                取消
              </button>
              <button
                onClick={() => handleDelete(deleteConfirm)}
                className="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
              >
                确认删除
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
