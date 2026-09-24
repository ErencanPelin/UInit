// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 28/01/2025
// -----------------------------------------------------

using System;
using UnityEngine.Events;

namespace Uinit.Core.Predicates
{
    public class UnityActionPredicate : IPredicate
    {
        private bool invoked;

        public UnityActionPredicate(Action<UnityAction> action)
        {
            action(Performed);
        }

        private void Performed()
        {
            invoked = true;
        }

        public bool Evaluate()
        {
            if (invoked)
            {
                invoked = false;
                return true;
            }

            invoked = false;
            return false;
        }
    }
}