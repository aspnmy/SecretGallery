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
    <div>
      <div className="mb-6 flex justify-between items-center">
        <h1 className="text-2xl font-bold">资源列表</h1>
        <div className="flex space-x-2">
          <button
            className={`px-4 py-2 rounded-md transition-colors ${albumType === 'video' ? 'bg-primary text-white' : 'bg-secondary text-foreground'}`}
            onClick={() => setAlbumType('video')}
          >
            视频相册
          </button>
          <button
            className={`px-4 py-2 rounded-md transition-colors ${albumType === 'image' ? 'bg-primary text-white' : 'bg-secondary text-foreground'}`}
            onClick={() => setAlbumType('image')}
          >
            图片相册
          </button>
        </div>
      </div>

      {albumType === 'image' ? (
        <PhotoAlbum
          layout="masonry"
          photos={photos}
          columns={4}
          spacing={16}
          defaultContainerWidth={1200}
          renderPhoto={(photo, { photoProps, imageProps, renderDefaultPhoto }) => (
            <a href={photo.href} className="group">
              {renderDefaultPhoto({ 
                photoProps: { 
                  ...photoProps,
                  className: `${photoProps.className} overflow-hidden rounded-lg shadow-sm transition-all duration-300 group-hover:shadow-md group-hover:scale-[1.02]`,
                },
                imageProps: { 
                  ...imageProps,
                  className: `${imageProps.className} object-cover`,
                  loading: 'lazy',
                },
              })}
              <div className="mt-2 text-sm text-center">
                {resources.find(r => r.id === parseInt(photo.id))?.title}
              </div>
            </a>
          )}
        />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {resources.map((resource) => (
            <a
              key={resource.id}
              href={`/resources/${resource.id}`}
              className="block group bg-secondary rounded-lg overflow-hidden shadow-sm transition-all duration-300 hover:shadow-md hover:-translate-y-1"
            >
              <div className="relative h-48 overflow-hidden">
                <Image
                  src={resource.poster_image || 'https://via.placeholder.com/600x400'}
                  alt={resource.title}
                  fill
                  className="object-cover transition-transform duration-500 group-hover:scale-110"
                  sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
                  priority
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent flex items-end">
                  <div className="p-4 text-white">
                    <h3 className="font-semibold text-lg">{resource.title}</h3>
                    <p className="text-sm text-white/80">{resource.resource_type || '未分类'}</p>
                  </div>
                </div>
              </div>
              <div className="p-4">
                <p className="text-sm line-clamp-2 text-muted-foreground mb-3">
                  {resource.description || '暂无简介'}
                </p>
                <div className="flex justify-between items-center text-xs text-muted-foreground">
                  <span>{resource.images?.length || 0} 张图片</span>
                  <span>{new Date(resource.created_at).toLocaleDateString()}</span>
                </div>
              </div>
            </a>
          ))}
        </div>
      )}

      {resources.length === 0 && (
        <div className="text-center py-20 text-muted-foreground">
          <p>暂无资源</p>
        </div>
      )}
    </div>
  );
}