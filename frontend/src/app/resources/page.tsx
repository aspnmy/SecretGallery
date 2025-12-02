'use client';

import { useState, useEffect } from 'react';
import { PhotoAlbum } from 'react-photo-album';
import Image from 'next/image';
import { Resource } from '@/types';
import { getResources } from '@/services/resourceService';

interface ResourcesPageProps {
  searchParams: {
    [key: string]: string | string[] | undefined;
  };
}

export default function ResourcesPage({ searchParams }: ResourcesPageProps) {
  const [resources, setResources] = useState<Resource[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [albumType, setAlbumType] = useState<'video' | 'image'>('video');

  useEffect(() => {
    const fetchResources = async () => {
      try {
        setLoading(true);
        const data = await getResources({ type: albumType });
        setResources(data);
      } catch (err) {
        setError('获取资源失败，请稍后重试');
        console.error('Failed to fetch resources:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchResources();
  }, [albumType]);

  if (loading) {
    return (
      <div className="flex justify-center items-center py-20">
        <div className="text-center">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-primary mb-4"></div>
          <p>加载中...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex justify-center items-center py-20">
        <div className="text-center text-red-500">
          <p>{error}</p>
          <button 
            onClick={() => window.location.reload()} 
            className="mt-4 px-4 py-2 bg-primary text-white rounded hover:bg-primary/90 transition-colors"
          >
            重试
          </button>
        </div>
      </div>
    );
  }

  // 转换资源数据为 PhotoAlbum 所需格式
  const photos = resources.map((resource) => ({
    id: resource.id.toString(),
    src: resource.poster_image || 'https://via.placeholder.com/600x400',
    width: 600,
    height: 400,
    alt: resource.title,
    href: `/resources/${resource.id}`,
  }));

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center flex-wrap gap-4">
        <h1 className="text-3xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent animate-fade-in">资源列表</h1>
        <div className="flex space-x-3 bg-background p-1.5 rounded-lg shadow-sm">
          <button
            className={`btn-custom ${albumType === 'video' ? 'btn-primary' : 'btn-outline'} flex-1 px-6 py-2 rounded-md transition-all`}
            onClick={() => setAlbumType('video')}
          >
            <span className="btn-text">视频相册</span>
          </button>
          <button
            className={`btn-custom ${albumType === 'image' ? 'btn-primary' : 'btn-outline'} flex-1 px-6 py-2 rounded-md transition-all`}
            onClick={() => setAlbumType('image')}
          >
            <span className="btn-text">图片相册</span>
          </button>
        </div>
      </div>

      {albumType === 'image' ? (
        <div className="card p-6 animate-slide-in-up">
          <PhotoAlbum
            layout="masonry"
            photos={photos}
            columns={4}
            spacing={16}
            defaultContainerWidth={1200}
          />
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 animate-slide-in-up">
          {resources.map((resource) => (
            <a
              key={resource.id}
              href={`/resources/${resource.id}`}
              className="card overflow-hidden group"
            >
              <div className="relative h-52 overflow-hidden">
                <Image
                  src={resource.poster_image || 'https://via.placeholder.com/600x400'}
                  alt={resource.title}
                  fill
                  className="object-cover transition-transform duration-700 group-hover:scale-110"
                  sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
                  priority
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/70 via-black/30 to-transparent flex items-end">
                  <div className="p-4 text-white">
                    <h3 className="font-semibold text-lg line-clamp-1">{resource.title}</h3>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="badge">{resource.resource_type || '未分类'}</span>
                      <span className="text-xs text-white/80">{new Date(resource.created_at).toLocaleDateString()}</span>
                    </div>
                  </div>
                </div>
              </div>
              <div className="p-5">
                <p className="text-sm line-clamp-2 text-dark mb-4">
                  {resource.description || '暂无简介'}
                </p>
                <div className="flex justify-between items-center text-xs">
                  <span className="text-gray flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                    </svg>
                    {resource.images?.length || 0} 张
                  </span>
                  {resource.tags && resource.tags.length > 0 && (
                    <span className="text-primary font-medium">{resource.tags.length} 标签</span>
                  )}
                </div>
              </div>
            </a>
          ))}
        </div>
      )}

      {resources.length === 0 && (
        <div className="text-center py-20 animate-fade-in">
          <div className="inline-block p-8 card mb-4">
            <svg className="w-16 h-16 text-gray" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <p className="text-xl text-gray font-medium">暂无资源</p>
          <p className="text-gray mt-2">快来提交第一个资源吧！</p>
          <a href="/submit" className="inline-block mt-4 btn btn-primary">
            <span className="btn-text">提交资源</span>
          </a>
        </div>
      )}
    </div>
  );
}