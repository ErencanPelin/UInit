// -----------------------------------------------------
//  Copyright (c) 2026 Erencan Pelin. All Rights Reserved.
// 
//  Author: Erencan Pelin
//  Date: 25/03/2026
//  -----------------------------------------------------

namespace Uinit.Core.DI
{
    public interface IProvider<in T>
    {
        public void Provide(T data);
    }
}