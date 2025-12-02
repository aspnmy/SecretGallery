'use client';

import { useState, useCallback } from 'react';
import { useDropzone } from 'react-dropzone';
import Compressor from 'compressorjs';
import { Resource, ImageInfo, VideoInfo } from '@/types';
import { createResource } from '@/services/resourceService';

interface SubmitPageProps {
  searchParams: { [key: string]: string | string[] | undefined };
}

export default function SubmitPage({ searchParams }: SubmitPageProps) {
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    author: '',
    source: '',
    resource_type: 'image',
    tags: [] as string[],
    video_url: '',
    poster_image: '',
  });
  
  const [images, setImages] = useState<ImageInfo[]>([]);
  const [videos, setVideos] = useState<VideoInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [tagInput, setTagInput] = useState('');
  const [videoSourceType, setVideoSourceType] = useState<'upload' | 'url'>('url'); // 视频来源类型

  /**
   * 处理图片压缩
   * @param file - 原始图片文件
   * @returns 压缩后的图片文件Promise
   */
  const compressImage = useCallback((file: File): Promise<File> => {
    return new Promise((resolve, reject) => {
      new Compressor(file, {
        quality: 0.8,
        maxWidth: 1920,
        maxHeight: 1080,
        mimeType: 'image/jpeg',
        success: (compressedFile) => {
          resolve(compressedFile as File);
        },
        error: (err) => {
          console.error('图片压缩失败:', err);
          reject(err);
        },
      });
    });
  }, []);

  /**
   * 处理视频文件
   * @param file - 原始视频文件
   * @returns 处理后的视频文件Promise
   */
  const processVideo = useCallback((file: File): Promise<File> => {
    return new Promise((resolve) => {
      // 视频压缩需要更复杂的处理，这里先直接返回原文件
      // 实际项目中可以使用ffmpeg.wasm或其他视频处理库
      resolve(file);
    });
  }, []);

  /**
   * 处理图片上传
   * @param files - 上传的文件列表
   */
  const onDropImage = useCallback(async (files: File[]) => {
    try {
      const uploadedImages: ImageInfo[] = [];
      
      for (const file of files) {
        // 只处理图片文件
        if (!file.type.startsWith('image/')) {
          continue;
        }

        // 压缩图片
        const compressedFile = await compressImage(file);
        
        // 创建临时URL用于预览
        const previewUrl = URL.createObjectURL(compressedFile);
        
        // 这里应该上传到服务器，返回实际URL
        // 暂时使用临时URL模拟
        uploadedImages.push({
          url: previewUrl,
          width: 0,
          height: 0,
          size: compressedFile.size,
          mime_type: compressedFile.type,
        });
      }
      
      setImages(prev => [...prev, ...uploadedImages]);
    } catch (err) {
      console.error('图片处理失败:', err);
      setError('图片处理失败，请重试');
    }
  }, [compressImage]);

  const { getRootProps: getImageRootProps, getInputProps: getImageInputProps, isDragActive: isImageDragActive } = useDropzone({
    onDrop: onDropImage,
    accept: {
      'image/*': ['.png', '.jpg', '.jpeg', '.gif', '.webp'],
    },
    multiple: true,
  });

  /**
   * 处理视频上传
   * @param files - 上传的文件列表
   */
  const onDropVideo = useCallback(async (files: File[]) => {
    try {
      const uploadedVideos: VideoInfo[] = [];
      
      for (const file of files) {
        // 只处理视频文件
        if (!file.type.startsWith('video/')) {
          continue;
        }

        // 处理视频文件
        const processedFile = await processVideo(file);
        
        // 创建临时URL用于预览
        const previewUrl = URL.createObjectURL(processedFile);
        
        // 这里应该上传到服务器，返回实际URL
        // 暂时使用临时URL模拟
        uploadedVideos.push({
          url: previewUrl,
          width: 0,
          height: 0,
          size: processedFile.size,
          mime_type: processedFile.type,
          is_local: true, // 标记为本地上传视频
        });
      }
      
      setVideos(prev => [...prev, ...uploadedVideos]);
    } catch (err) {
      console.error('视频处理失败:', err);
      setError('视频处理失败，请重试');
    }
  }, [processVideo]);

  const { getRootProps: getVideoRootProps, getInputProps: getVideoInputProps, isDragActive: isVideoDragActive } = useDropzone({
    onDrop: onDropVideo,
    accept: {
      'video/*': ['.mp4', '.avi', '.mov', '.wmv', '.flv', '.webm'],
    },
    multiple: true,
  });

  /**
   * 添加标签
   */
  const addTag = () => {
    if (tagInput.trim() && !formData.tags.includes(tagInput.trim())) {
      setFormData(prev => ({
        ...prev,
        tags: [...prev.tags, tagInput.trim()],
      }));
      setTagInput('');
    }
  };

  /**
   * 删除标签
   * @param tag - 要删除的标签
   */
  const removeTag = (tag: string) => {
    setFormData(prev => ({
      ...prev,
      tags: prev.tags.filter(t => t !== tag),
    }));
  };

  /**
   * 处理表单提交
   * @param e - 表单提交事件
   */
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!formData.title) {
      setError('请输入资源标题');
      return;
    }

    if (formData.resource_type === 'video') {
      if (videoSourceType === 'url' && !formData.video_url) {
        setError('视频资源请输入视频URL');
        return;
      }
      if (videoSourceType === 'upload' && videos.length === 0) {
        setError('视频资源请上传至少一个视频');
        return;
      }
    }

    if (formData.resource_type === 'image' && images.length === 0) {
      setError('图片资源请上传至少一张图片');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      // 构建视频数据
      const videoData: VideoInfo[] = [];
      
      // 如果是URL视频，添加到视频数组
      if (videoSourceType === 'url' && formData.video_url) {
        videoData.push({
          url: formData.video_url,
          width: 0,
          height: 0,
          size: 0,
          mime_type: 'video/mp4', // 默认类型
          is_local: false, // URL视频
        });
      }
      
      // 添加上传的视频
      if (videoSourceType === 'upload') {
        videoData.push(...videos);
      }

      // 这里应该上传文件到服务器，获取实际URL
      // 暂时使用模拟数据
      const resourceData = {
        ...formData,
        title_en: formData.title, // 默认使用title作为title_en
        images: formData.resource_type === 'image' ? images : [],
        videos: videoData, // 使用视频数组代替单个video_url
        video_url: undefined, // 移除旧的video_url字段
        links: { // 添加默认链接对象
          magnet: [],
          ed2k: [],
          uc: [],
          mobile: [],
          tianyi: [],
          quark: [],
          '115': [],
          aliyun: [],
          pikpak: [],
          baidu: [],
          '123': [],
          xunlei: [],
          online: [],
          others: [],
        },
        tmdb_id: null, // 默认值
        stickers: [], // 默认值
        media_type: formData.resource_type as 'video' | 'image', // 根据resource_type设置
        liked_by: [], // 默认值
        is_approved: false, // 默认值
      };

      await createResource(resourceData as Omit<Resource, 'id' | 'created_at' | 'updated_at'>);
      
      setSuccess('资源提交成功！');
      
      // 重置表单
      setFormData({
        title: '',
        description: '',
        author: '',
        source: '',
        resource_type: 'image',
        tags: [],
        video_url: '',
        poster_image: '',
      });
      setImages([]);
      setVideos([]);
      setVideoSourceType('url');
    } catch (err) {
      console.error('资源提交失败:', err);
      setError('资源提交失败，请稍后重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent animate-fade-in-up">提交资源</h1>
      
      {error && (
        <div className="mb-6 p-4 bg-danger/10 text-danger rounded-lg">
          {error}
        </div>
      )}
      
      {success && (
        <div className="mb-6 p-4 bg-success/10 text-success rounded-lg">
          {success}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-6 animate-slide-in-up">
        {/* 基本信息 */}
        <div className="card p-6">
          <h2 className="text-2xl font-semibold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">基本信息</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="col-span-2">
              <label htmlFor="title" className="form-label">标题 *</label>
              <input
                type="text"
                id="title"
                value={formData.title}
                onChange={(e) => setFormData(prev => ({ ...prev, title: e.target.value }))}
                className="form-control"
                placeholder="请输入资源标题"
              />
            </div>

            <div className="col-span-2">
              <label htmlFor="description" className="form-label">描述</label>
              <textarea
                id="description"
                value={formData.description}
                onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                className="form-control"
                rows={4}
                placeholder="请输入资源描述"
              />
            </div>

            <div>
              <label htmlFor="author" className="form-label">作者</label>
              <input
                type="text"
                id="author"
                value={formData.author}
                onChange={(e) => setFormData(prev => ({ ...prev, author: e.target.value }))}
                className="form-control"
                placeholder="请输入作者"
              />
            </div>

            <div>
              <label htmlFor="source" className="form-label">来源</label>
              <input
                type="text"
                id="source"
                value={formData.source}
                onChange={(e) => setFormData(prev => ({ ...prev, source: e.target.value }))}
                className="form-control"
                placeholder="请输入来源"
              />
            </div>

            <div>
              <label htmlFor="resource_type" className="form-label">资源类型 *</label>
              <select
                id="resource_type"
                value={formData.resource_type}
                onChange={(e) => setFormData(prev => ({ ...prev, resource_type: e.target.value }))}
                className="form-control"
              >
                <option value="image">图片</option>
                <option value="video">视频</option>
              </select>
            </div>
          </div>
        </div>

        {/* 视频信息 */}
        {formData.resource_type === 'video' && (
          <div className="card p-6">
            <h2 className="text-2xl font-semibold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">视频信息</h2>
            
            <div className="space-y-6">
              {/* 视频来源类型选择 */}
              <div>
                <label className="form-label">视频来源类型 *</label>
                <div className="flex gap-4">
                  <label className="flex items-center cursor-pointer">
                    <input
                      type="radio"
                      name="videoSourceType"
                      value="url"
                      checked={videoSourceType === 'url'}
                      onChange={() => setVideoSourceType('url')}
                      className="mr-2"
                    />
                    <span>URL链接</span>
                  </label>
                  <label className="flex items-center cursor-pointer">
                    <input
                      type="radio"
                      name="videoSourceType"
                      value="upload"
                      checked={videoSourceType === 'upload'}
                      onChange={() => setVideoSourceType('upload')}
                      className="mr-2"
                    />
                    <span>本地上传</span>
                  </label>
                </div>
              </div>
              
              {/* URL视频输入 */}
              {videoSourceType === 'url' && (
                <div>
                  <label htmlFor="video_url" className="form-label">视频URL *</label>
                  <input
                    type="url"
                    id="video_url"
                    value={formData.video_url}
                    onChange={(e) => setFormData(prev => ({ ...prev, video_url: e.target.value }))}
                    className="form-control"
                    placeholder="请输入视频URL"
                  />
                </div>
              )}
              
              {/* 本地上传视频 */}
              {videoSourceType === 'upload' && (
                <div>
                  <label className="form-label">视频上传 *</label>
                  <div
                    {...getVideoRootProps()}
                    className={`border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors ${isVideoDragActive ? 'border-primary bg-primary/10' : 'border-border hover:border-primary hover:bg-primary/5'}`}
                  >
                    <input {...getVideoInputProps()} />
                    {isVideoDragActive ? (
                      <p className="text-lg font-medium text-primary">释放鼠标上传视频</p>
                    ) : (
                      <p className="text-lg font-medium">拖拽视频到此处，或点击选择视频</p>
                    )}
                    <p className="text-sm text-gray mt-2">支持 MP4, AVI, MOV, WMV, FLV, WebM 格式</p>
                  </div>
                  
                  {/* 已上传视频列表 */}
                  {videos.length > 0 && (
                    <div className="mt-6">
                      <h3 className="text-xl font-medium mb-4">已上传视频 ({videos.length})</h3>
                      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                        {videos.map((video, index) => (
                          <div key={index} className="relative group card p-0 overflow-hidden">
                            <div className="aspect-video bg-background">
                              <video
                                  src={video.url}
                                  className="w-full h-full object-cover"
                                  controls={false}
                                  muted
                                  loop
                                  playsInline
                                  aria-label={`上传视频 ${index + 1}`}
                                />
                            </div>
                            <button
                              type="button"
                              onClick={() => setVideos(prev => prev.filter((_, i) => i !== index))}
                              className="absolute -top-2 -right-2 bg-danger text-white rounded-full w-7 h-7 flex items-center justify-center text-sm opacity-0 group-hover:opacity-100 transition-opacity hover:bg-danger/90"
                            >
                              ×
                            </button>
                            <div className="p-3 bg-background">
                              <div className="text-xs text-gray">
                                {Math.round(video.size / 1024)}KB
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
              
              <div>
                <label htmlFor="poster_image" className="form-label">封面图片URL</label>
                <input
                  type="url"
                  id="poster_image"
                  value={formData.poster_image}
                  onChange={(e) => setFormData(prev => ({ ...prev, poster_image: e.target.value }))}
                  className="form-control"
                  placeholder="请输入封面图片URL"
                />
              </div>
            </div>
          </div>
        )}

        {/* 图片上传 */}
        {formData.resource_type === 'image' && (
          <div className="card p-6">
            <h2 className="text-2xl font-semibold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">图片上传</h2>
            
            <div
              {...getImageRootProps()}
              className={`border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors ${isImageDragActive ? 'border-primary bg-primary/10' : 'border-border hover:border-primary hover:bg-primary/5'}`}
            >
              <input {...getImageInputProps()} />
              {isImageDragActive ? (
                <p className="text-lg font-medium text-primary">释放鼠标上传图片</p>
              ) : (
                <p className="text-lg font-medium">拖拽图片到此处，或点击选择图片</p>
              )}
              <p className="text-sm text-gray mt-2">支持 PNG, JPG, JPEG, GIF, WebP 格式</p>
              <p className="text-sm text-gray">图片将自动压缩至合适大小</p>
            </div>

            {images.length > 0 && (
              <div className="mt-6">
                <h3 className="text-xl font-medium mb-4">已上传图片 ({images.length})</h3>
                <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                  {images.map((image, index) => (
                    <div key={index} className="relative group card p-0 overflow-hidden">
                      <div className="aspect-square bg-background">
                        <img
                          src={image.url}
                          alt={`上传图片 ${index + 1}`}
                          className="w-full h-full object-cover"
                          onError={(e) => {
                            const target = e.target as HTMLImageElement;
                            target.src = '/placeholder-image.svg';
                            target.alt = `默认图片 ${index + 1}`;
                          }}
                        />
                      </div>
                      <button
                        type="button"
                        onClick={() => setImages(prev => prev.filter((_, i) => i !== index))}
                        className="absolute -top-2 -right-2 bg-danger text-white rounded-full w-7 h-7 flex items-center justify-center text-sm opacity-0 group-hover:opacity-100 transition-opacity hover:bg-danger/90"
                      >
                        ×
                      </button>
                      <div className="p-3 bg-background">
                        <div className="text-xs text-gray">
                          {Math.round(image.size / 1024)}KB
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {/* 标签 */}
        <div className="card p-6">
          <h2 className="text-2xl font-semibold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">标签</h2>
          
          <div className="flex items-center gap-3 mb-4">
            <input
              type="text"
              id="tagInput"
              value={tagInput}
              onChange={(e) => setTagInput(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && addTag()}
              className="flex-1 form-control"
              placeholder="输入标签，按回车添加"
            />
            <button
              type="button"
              onClick={addTag}
              className="btn btn-primary px-6 py-2"
            >
              <span className="btn-text">添加</span>
            </button>
          </div>
          
          {formData.tags.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {formData.tags.map((tag, index) => (
                <span key={index} className="tag">
                  {tag}
                  <button
                    type="button"
                    onClick={() => removeTag(tag)}
                    className="ml-2 text-primary/80 hover:text-primary"
                  >
                    ×
                  </button>
                </span>
              ))}
            </div>
          )}
        </div>

        {/* 提交按钮 */}
        <div className="flex justify-center">
          <button
            type="submit"
            disabled={loading}
            className="btn btn-primary px-10 py-3"
          >
            {loading ? (
              <>
                <div className="loading mr-2"></div>
                <span className="btn-text">提交中...</span>
              </>
            ) : (
              <span className="btn-text">提交资源</span>
            )}
          </button>
        </div>
      </form>
    </div>
  );
}
