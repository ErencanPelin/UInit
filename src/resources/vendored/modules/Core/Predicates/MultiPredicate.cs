// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 28/01/2025
// -----------------------------------------------------

namespace UinitCoreUtils.Predicates
{
    public class MultiPredicate : IPredicate
    {
        private readonly IPredicate conditionA;
        private readonly IPredicate conditionB;

        public MultiPredicate(IPredicate conditionA, IPredicate conditionB)
        {
            this.conditionA = conditionA;
            this.conditionB = conditionB;
        }

        public bool Evaluate() => conditionA.Evaluate() && conditionB.Evaluate();
    }
}