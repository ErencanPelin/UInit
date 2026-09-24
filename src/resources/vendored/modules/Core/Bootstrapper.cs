// -----------------------------------------------------
//  Copyright (c) 2026 Erencan Pelin. All Rights Reserved.
// 
//  Author: Erencan Pelin
//  Date: 25/03/2026
//  -----------------------------------------------------

namespace UinitCoreUtils.Core
{
    /// <summary>
    /// Bootstraps all the bootstrappers attached to this GameObject in deterministic order
    /// </summary>
    public class Bootstrapper : MonoBehaviour
    {
        private IBootstrapper[] _bootstrappers;

        private void Awake()
        {
            var bootstrappers = GetComponents<IBootstrapper>();
            _bootstrappers = bootstrappers.OrderBy(x => x.Order).ToArray();

            //loop through each bootstrapper and strap them
            foreach (var bootstrapper in bootstrappers)
                bootstrapper.Bootstrap();
        }

        private void OnDestroy()
        {
            //loop through each bootstrapper and unstrap them in reverse order
            foreach (var bootstrapper in _bootstrappers.Reverse())
                bootstrapper.Unstrap();
        }
    }
}