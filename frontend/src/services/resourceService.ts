// src/services/resourceService.ts
// 资源服务，处理资源相关的API请求

import axios from 'axios';
import { Resource, GetResourcesRequest, GetResourcesResponse } from '@/types';

// 创建axios实例
const apiClient = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

/**
 * 获取资源列表
 * @param params - 请求参数，包含类型、分页等信息
 * @returns 资源列表数据
 */
export const getResources = async (params: GetResourcesRequest): Promise<Resource[]> => {
  try {
    const response = await apiClient.get<GetResourcesResponse>('/resources', {
      params,
    });
    return response.data.data;
  } catch (error) {
    console.error('获取资源列表失败:', error);
    throw error;
  }
};

/**
 * 获取资源详情
 * @param id - 资源ID
 * @returns 资源详情数据
 */
export const getResourceById = async (id: number): Promise<Resource> => {
  try {
    const response = await apiClient.get<{ data: Resource }>(`/resources/${id}`);
    return response.data.data;
  } catch (error) {
    console.error(`获取资源 ${id} 详情失败:`, error);
    throw error;
  }
};

/**
 * 创建新资源
 * @param resource - 资源数据
 * @returns 创建的资源数据
 */
export const createResource = async (resource: Omit<Resource, 'id' | 'created_at' | 'updated_at'>): Promise<Resource> => {
  try {
    const response = await apiClient.post<{ data: Resource }>('/resources', resource);
    return response.data.data;
  } catch (error) {
    console.error('创建资源失败:', error);
    throw error;
  }
};

/**
 * 更新资源
 * @param id - 资源ID
 * @param resource - 更新的资源数据
 * @returns 更新后的资源数据
 */
export const updateResource = async (id: number, resource: Partial<Resource>): Promise<Resource> => {
  try {
    const response = await apiClient.put<{ data: Resource }>(`/resources/${id}`, resource);
    return response.data.data;
  } catch (error) {
    console.error(`更新资源 ${id} 失败:`, error);
    throw error;
  }
};

/**
 * 删除资源
 * @param id - 资源ID
 * @returns 删除结果
 */
export const deleteResource = async (id: number): Promise<boolean> => {
  try {
    await apiClient.delete(`/resources/${id}`);
    return true;
  } catch (error) {
    console.error(`删除资源 ${id} 失败:`, error);
    throw error;
  }
};
