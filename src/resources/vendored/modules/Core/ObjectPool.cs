// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 20/09/2025
// -----------------------------------------------------

#nullable enable
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;

namespace UinitCoreUtils.Core
{
    public abstract class ObjectPool<TComponent, TData> where TComponent : Component
    {
        private readonly GameObject poolObjectPrefab = null!;
        private readonly int numItems;
        private readonly Queue<TComponent> objectPool = new();
        protected readonly GameObject poolManagerObject;

        public ObjectPool(GameObject poolObjectPrefab, int numItems)
        {
            this.poolObjectPrefab = poolObjectPrefab;
            this.numItems = numItems;

            var poolManagerGameObject = new GameObject($"{typeof(TComponent).Name} Pool");
            poolManagerObject = poolManagerGameObject;

            CreatePool();
        }

        //loop through and create all the objects required for our pool
        private void CreatePool()
        {
            var _transform = poolManagerObject.transform; //store transform in cache because transform = GetComponent call

            for (var i = 0; i < numItems; i++)
            {
                var pooledObject = Object.Instantiate(poolObjectPrefab, _transform);
                pooledObject.SetActive(false);
                objectPool.Enqueue(pooledObject.GetComponent<TComponent>());
            }
        }

        protected TComponent? TrySpawnPooledObject(Vector3 spawnPosition)
        {
            if (objectPool.Count is 0) return null;
            var pooledObject = objectPool.Dequeue();
            pooledObject.transform.position = spawnPosition;
            pooledObject.gameObject.SetActive(true);
            return pooledObject;
        }

        protected void ReturnToQueue(TComponent pooledObject)
        {
            pooledObject.gameObject.SetActive(false);
            objectPool.Enqueue(pooledObject);
        }

        protected async Task ReturnToQueueAfterTime(TComponent poolObject, float time)
        {
            await Task.Delay((int)(time * 1000));
            ReturnToQueue(poolObject);
        }
    }
}