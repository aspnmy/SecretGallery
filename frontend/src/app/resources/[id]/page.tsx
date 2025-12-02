'use client';

import { useState, useEffect } from 'react';
import { useParams } from 'next/navigation';
import { Resource } from '@/types';
import { getResourceById } from '@/services/resourceService';
import { PhotoAlbum } from 'react-photo-album';
import Image from 'next/image';

interface ResourceDetailPageProps {
  searchParams: { [key: string]: string | string[] | undefined };
}

export default function ResourceDetailPage({ searchParams }: ResourceDetailPageProps) {
  const params = useParams();
  const id = parseInt(params.id as string);
  const [resource, setResource] = useState<Resource | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedImageIndex, setSelectedImageIndex] = useState(0);

  useEffect(() => {
    const fetchResource = async () => {
      try {
        setLoading(true);
        const data = await getResourceById(id);
        setResource(data);
      } catch (err) {
        setError('获取资源详情失败，请稍后重试');
        console.error('Failed to fetch resource:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchResource();
  }, [id]);

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

  if (error || !resource) {
    return (
      <div className="flex justify-center items-center py-20">
        <div className="text-center text-red-500">
          <p>{error || '资源不存在'}</p>
          <button 
            onClick={() => window.history.back()} 
            className="mt-4 px-4 py-2 bg-primary text-white rounded hover:bg-primary/90 transition-colors"
          >
            返回
          </button>
        </div>
      </div>
    );
  }

  // 准备媒体数据用于PhotoAlbum（支持视频和图片）
  const mediaItems = [
    // 添加视频
    ...(resource.videos?.map((video, index) => ({
      id: `video-${index}`,
      src: video.url,
      width: video.width || 600,
      height: video.height || 400,
      alt: `${resource.title} - 视频 ${index + 1}`,
      type: 'video',
      is_local: video.is_local,
      mime_type: video.mime_type,
    })) || []),
    // 添加图片
    ...(resource.images?.map((image, index) => ({
      id: `image-${index}`,
      src: image.url,
      width: image.width || 600,
      height: image.height || 400,
      alt: `${resource.title} - 图片 ${index + 1}`,
      type: 'image',
    })) || [])
  ];

  return (
    <div className="max-w-6xl mx-auto">
      <div className="mb-6">
        <button 
          onClick={() => window.history.back()} 
          className="inline-flex items-center text-primary hover:underline mb-4"
        >
          ← 返回列表
        </button>
        <h1 className="text-3xl font-bold mb-2">{resource.title}</h1>
        <div className="flex flex-wrap gap-2 mb-4">
          <span className="px-2 py-1 bg-secondary rounded text-sm">{resource.resource_type}</span>
          <span className="px-2 py-1 bg-secondary rounded text-sm">{resource.source}</span>
          <span className="px-2 py-1 bg-secondary rounded text-sm">
            {new Date(resource.created_at).toLocaleDateString()}
          </span>
        </div>
        <p className="text-muted mb-6">{resource.description}</p>
      </div>

      {mediaItems.length > 0 && (
        <div className="mb-10">
          <h2 className="text-xl font-semibold mb-4">媒体画廊</h2>
          <PhotoAlbum
            layout="masonry"
            photos={mediaItems}
            columns={4}
            spacing={12}
            renderPhoto={(photo, photoProps) => {
              // 根据媒体类型渲染不同的组件
              if (photo.type === 'video') {
                return (
                  <div
                    key={photo.id}
                    {...photoProps}
                    className="relative group"
                  >
                    <div className="aspect-[16/9] bg-black rounded-lg overflow-hidden">
                      <video
                        src={photo.src}
                        alt={photo.alt}
                        className="w-full h-full object-cover"
                        poster={resource.poster_image}
                        muted
                        loop
                        playsInline
                      />
                      <div className="absolute inset-0 flex items-center justify-center bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity">
                        <div className="w-16 h-16 bg-white/30 backdrop-blur-sm rounded-full flex items-center justify-center">
                          <div className="w-10 h-10 bg-white rounded-full flex items-center justify-center">
                            <span className="text-black text-2xl">▶</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                );
              }
              // 图片渲染
              return (
                <div
                  key={photo.id}
                  {...photoProps}
                  className="relative group"
                >
                  <Image
                    src={photo.src}
                    alt={photo.alt}
                    width={photo.width}
                    height={photo.height}
                    className="w-full h-full object-cover rounded-lg"
                    priority
                  />
                </div>
              );
            }}
          />

          <div className="mt-6 border-t pt-6">
            <h3 className="text-lg font-medium mb-3">选中媒体</h3>
            <div className="flex justify-center">
              <div className="max-w-3xl bg-secondary p-4 rounded-lg shadow">
                {mediaItems[selectedImageIndex]?.type === 'video' ? (
                  <video
                    src={mediaItems[selectedImageIndex]?.src || ''}
                    alt={mediaItems[selectedImageIndex]?.alt || ''}
                    className="rounded-lg max-w-full h-auto object-contain"
                    controls
                    poster={resource.poster_image}
                  >
                    您的浏览器不支持视频播放
                  </video>
                ) : (
                  <Image
                    src={mediaItems[selectedImageIndex]?.src || ''}
                    alt={mediaItems[selectedImageIndex]?.alt || ''}
                    width={mediaItems[selectedImageIndex]?.width || 1200}
                    height={mediaItems[selectedImageIndex]?.height || 800}
                    className="rounded-lg max-w-full h-auto object-contain"
                    priority
                  />
                )}
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="bg-secondary rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">资源信息</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="flex flex-col">
            <span className="text-sm text-muted">作者</span>
            <span className="font-medium">{resource.author || '未知'}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm text-muted">来源</span>
            <span className="font-medium">{resource.source}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm text-muted">分类</span>
            <span className="font-medium">{resource.resource_type}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm text-muted">创建时间</span>
            <span className="font-medium">{new Date(resource.created_at).toLocaleString()}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm text-muted">更新时间</span>
            <span className="font-medium">{new Date(resource.updated_at).toLocaleString()}</span>
          </div>
          {resource.tags && resource.tags.length > 0 && (
            <div className="flex flex-col md:col-span-2">
              <span className="text-sm text-muted">标签</span>
              <div className="flex flex-wrap gap-2 mt-1">
                {resource.tags.map((tag, index) => (
                  <span key={index} className="px-2 py-1 bg-background rounded text-sm">{tag}</span>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
