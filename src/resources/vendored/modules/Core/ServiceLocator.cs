// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 20/09/2025
// -----------------------------------------------------

using System;
using System.Collections.Generic;

namespace Uinit.Core
{
    public class ServiceLocator
    {
        private static ServiceLocator _instance;
        private readonly Dictionary<Type, object> _services = new();

        public static ServiceLocator Instance => _instance ??= new ServiceLocator();

        /// <summary>
        /// Registers a service
        /// </summary>
        public void Register<T>(T service) where T : class
        {
            var type = typeof(T);
            if (_services.ContainsKey(type)) throw new Exception($"Service {type} is already registered!");
            _services.Add(type, service);
        }

        /// <summary>
        /// Unregisters an existing service
        /// </summary>
        public void Unregister<T>() where T : class
        {
            var type = typeof(T);
            if (!_services.ContainsKey(type)) throw new NullReferenceException($"Service {type} is not registered");
            _services.Remove(type);
        }

        /// <summary>
        /// Returns the instance of the service based on the given service type
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <returns>The instance of the service of the given type</returns>
        public T Resolve<T>() where T : class
        {
            var type = typeof(T);
            if (_services.TryGetValue(type, out var service)) return (T)service;
            throw new NullReferenceException($"Service {type} not found!");
        }

        /// <summary>
        /// Returns true if that service is registered already
        /// </summary>
        public bool IsRegistered<T>() where T : class
        {
            return _services.ContainsKey(typeof(T));
        }
    }
}