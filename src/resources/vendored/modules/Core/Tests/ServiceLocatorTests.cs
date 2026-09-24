// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 14/02/2025
// -----------------------------------------------------

using System;
using NUnit.Framework;
using Uinit.Core;

namespace Uinit.Core.Tests
{
    [TestFixture]
    public class ServiceLocatorTests
    {
        private interface ITestService
        {
        }

        private class TestService : ITestService
        {
        }

        private TestService testService;
        private ServiceLocator testServiceLocator;

        [OneTimeSetUp]
        public void OneTimeSetUp()
        {
            testService = new TestService();
        }

        [SetUp]
        public void Setup()
        {
            //create a clean service locator so that we don't need to clean up service locators after registering services
            testServiceLocator = new ServiceLocator();
        }

        [Test]
        public void trying_to_unregister_service_when_not_registered_should_throw_exception()
        {
            Assert.Throws<NullReferenceException>(() => testServiceLocator.Unregister<ITestService>());
        }

        [Test]
        public void trying_to_register_existing_service_should_throw_exception()
        {
            testServiceLocator.Register<ITestService>(testService);
            Assert.Throws<Exception>(() => testServiceLocator.Register<ITestService>(testService));
        }

        [Test]
        public void isRegistered_should_return_TRUE_if_service_IS_registered()
        {
            testServiceLocator.Register<ITestService>(testService);
            Assert.IsTrue(testServiceLocator.IsRegistered<ITestService>());
        }

        [Test]
        public void isRegistered_should_return_FALSE_if_service_NOT_registered()
        {
            Assert.IsFalse(testServiceLocator.IsRegistered<ITestService>());
        }

        [Test]
        public void deregistering_service_should_remove_service()
        {
            testServiceLocator.Register<ITestService>(testService);
            Assert.IsTrue(testServiceLocator.IsRegistered<ITestService>());
            testServiceLocator.Unregister<ITestService>();
            Assert.IsFalse(testServiceLocator.IsRegistered<ITestService>());
        }

        [Test]
        public void registering_service_should_register_correct_service_to_service_locator()
        {
            testServiceLocator.Register<ITestService>(testService);
            Assert.IsTrue(testServiceLocator.IsRegistered<ITestService>());

            testServiceLocator.Unregister<ITestService>();
        }

        [Test]
        public void service_locator_should_return_correct_service_reference_when_GET()
        {
            testServiceLocator.Register<ITestService>(testService);
            Assert.AreSame(testService, testServiceLocator.Resolve<ITestService>());
        }
    }
}