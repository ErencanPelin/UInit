// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 20/09/2025
// -----------------------------------------------------

using UnityEngine;
using UnityEngine.Events;

namespace UinitCoreUtils.Core
{
    [System.Serializable]
    public class Observable<T>
    {
        [SerializeField] private T value;
        [SerializeField] private UnityEvent<T> onValueChanged;

        private event UnityAction<T> ValueChanged;

        public T Value
        {
            get => value;
            set => Set(value);
        }

        public static implicit operator T(Observable<T> observable) => observable.value;

        public Observable(T value, UnityAction<T> callback = null)
        {
            this.value = value;
            ValueChanged = delegate { };
            onValueChanged = new UnityEvent<T>();
            ValueChanged += onValueChanged.Invoke;
            if (callback != null) ValueChanged += callback;
        }

        private void Set(T value)
        {
            if (Equals(this.value, value)) return;
            this.value = value;
            Invoke();
        }

        public void SetValueSilently(T value)
            => this.value = value;

        public void Invoke()
        {
            ValueChanged?.Invoke(value);
        }

        public void AddListener(UnityAction<T> callback)
        {
            if (callback == null) return;
            if (ValueChanged == null) ValueChanged = delegate { };

            ValueChanged += callback;
        }

        public void RemoveListener(UnityAction<T> callback)
        {
            if (callback == null) return;
            if (ValueChanged == null) ValueChanged = delegate { };

            ValueChanged -= callback;
        }

        public void RemoveAllListeners()
        {
            if (onValueChanged == null) return;

            onValueChanged.RemoveAllListeners();
            ValueChanged = delegate { };
        }

        public void Dispose()
        {
            RemoveAllListeners();
            onValueChanged = null;
            ValueChanged = null;
            value = default;
        }
    }
}