// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 28/01/2025
// -----------------------------------------------------

using System;

namespace UinitCoreUtils.Predicates
{
    public class FuncPredicate : IPredicate
    {
        private readonly Func<bool> func;

        public FuncPredicate(Func<bool> func)
        {
            this.func = func;
        }

        public bool Evaluate() => func.Invoke();
    }
}