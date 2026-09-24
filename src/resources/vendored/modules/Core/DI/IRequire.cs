// -----------------------------------------------------
//  Copyright (c) 2026 Erencan Pelin. All Rights Reserved.
// 
//  Author: Erencan Pelin
//  Date: 25/03/2026
//  -----------------------------------------------------

namespace UinitCoreUtils.Core.DI
{
    public interface IRequire<in T>
    {
        public void SetRef(T data);
    }
}