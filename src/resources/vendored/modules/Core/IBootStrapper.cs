// -----------------------------------------------------
//  Copyright (c) 2026 Erencan Pelin. All Rights Reserved.
// 
//  Author: Erencan Pelin
//  Date: 25/03/2026
//  -----------------------------------------------------

namespace UinitCoreUtils.Core
{
    /// <summary>
    /// implemented by classes that bootstrap different domains. The core bootstrapper loops through each bootstrapper
    /// in order so that cross-domain dependencies are resolved in an explicit order.
    /// </summary>
    public interface IBootstrapper
    {
        public int Order { get; }
        public void Bootstrap();
        public void Unstrap();
    }
}