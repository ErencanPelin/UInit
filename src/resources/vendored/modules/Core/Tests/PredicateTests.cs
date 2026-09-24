// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 14/02/2025
// -----------------------------------------------------

using NUnit.Framework;
using UnityEngine.Events;
using Uinit.Core.Predicates;

namespace Uinit.Core.Tests
{
    [TestFixture]
    public class PredicateTests
    {
        [Test]
        public void FUNC_predicate_should_return_TRUE_when_condition_is_TRUE()
        {
            var condition = false;
            var funcPredicate = new FuncPredicate(() => condition);
            Assert.IsFalse(funcPredicate.Evaluate());
            condition = true;
            Assert.IsTrue(funcPredicate.Evaluate());
        }

        [Test]
        public void UNITYACTION_predicate_should_return_TRUE_when_action_INVOKED()
        {
            var unityAction = new UnityAction(() => { });
            var actionPredicate = new UnityActionPredicate(action => unityAction += action);
            Assert.IsFalse(actionPredicate.Evaluate());
            unityAction.Invoke();
            Assert.IsTrue(actionPredicate.Evaluate());
            //action should reset to false after evaluated
            Assert.IsFalse(actionPredicate.Evaluate());
        }

        [Test]
        public void MULTI_predicate_should_return_FALSE_when_BOTH_predicates_are_NOT_TRUE()
        {
            var conditionA = false;
            var conditionB = false;
            var funcPredicateA = new FuncPredicate(() => conditionA);
            var funcPredicateB = new FuncPredicate(() => conditionB);
            var multiPredicate = new MultiPredicate(funcPredicateA, funcPredicateB);

            Assert.IsFalse(multiPredicate.Evaluate());

            conditionA = true;
            Assert.IsFalse(multiPredicate.Evaluate());
        }

        [Test]
        public void MULTI_predicate_should_return_FALSE_when_BOTH_predicates_ARE_TRUE()
        {
            var conditionA = false;
            var conditionB = false;
            var funcPredicateA = new FuncPredicate(() => conditionA);
            var funcPredicateB = new FuncPredicate(() => conditionB);
            var multiPredicate = new MultiPredicate(funcPredicateA, funcPredicateB);

            Assert.IsFalse(multiPredicate.Evaluate());

            conditionA = true;
            conditionB = true;
            Assert.IsTrue(multiPredicate.Evaluate());
        }

        [Test]
        public void NESTED_MULTI_predicate_should_return_TRUE_when_ALL_predicates_ARE_NOT_TRUE()
        {
            var conditionA = false;
            var conditionB = false;
            var conditionC = false;
            var conditionD = false;
            var funcPredicateA = new FuncPredicate(() => conditionA);
            var funcPredicateB = new FuncPredicate(() => conditionB);
            var funcPredicateC = new FuncPredicate(() => conditionC);
            var funcPredicateD = new FuncPredicate(() => conditionD);

            var multiPredicateA = new MultiPredicate(funcPredicateA, funcPredicateB);
            var multiPredicateB = new MultiPredicate(funcPredicateC, funcPredicateD);

            var nestedMultiPredicate = new MultiPredicate(multiPredicateA, multiPredicateB);

            Assert.IsFalse(nestedMultiPredicate.Evaluate());

            conditionA = true;
            conditionB = true;
            Assert.IsFalse(nestedMultiPredicate.Evaluate());
        }

        [Test]
        public void NESTED_MULTI_predicate_should_return_TRUE_when_ALL_predicates_ARE_TRUE()
        {
            var conditionA = false;
            var conditionB = false;
            var conditionC = false;
            var conditionD = false;
            var funcPredicateA = new FuncPredicate(() => conditionA);
            var funcPredicateB = new FuncPredicate(() => conditionB);
            var funcPredicateC = new FuncPredicate(() => conditionC);
            var funcPredicateD = new FuncPredicate(() => conditionD);

            var multiPredicateA = new MultiPredicate(funcPredicateA, funcPredicateB);
            var multiPredicateB = new MultiPredicate(funcPredicateC, funcPredicateD);

            var nestedMultiPredicate = new MultiPredicate(multiPredicateA, multiPredicateB);

            Assert.IsFalse(nestedMultiPredicate.Evaluate());

            conditionA = true;
            conditionB = true;
            conditionC = true;
            conditionD = true;
            Assert.IsTrue(nestedMultiPredicate.Evaluate());
        }
    }
}