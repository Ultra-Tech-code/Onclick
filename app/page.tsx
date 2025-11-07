'use client';

import { motion } from 'framer-motion';
import Navbar from '../components/Navbar';
import Footer from '../components/Footer';
import { ArrowRight, Sparkles, Users, Building2, Target, Zap, Globe, Shield, CheckCircle } from 'lucide-react';
import Link from 'next/link';

export default function LandingPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
      <Navbar />
      
      {/* Hero Section */}
      <section className="relative pt-32 pb-20 px-4 sm:px-6 lg:px-8 overflow-hidden">
        {/* Video Background */}
        <div className="absolute inset-0 w-full h-full overflow-hidden">
          <video
            autoPlay
            muted
            loop
            playsInline
            className="w-full h-full object-contain"
            style={{
              minWidth: '100%',
              minHeight: '100%',
              width: 'auto',
              height: 'auto',
              position: 'absolute',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)'
            }}
          >
            <source src="https://res.cloudinary.com/dxswouxj5/video/upload/v1761270457/6913299_Motion_Graphics_Motion_Graphic_1080x1920_u1bpqe.mp4" type="video/mp4" />
          </video>
          <div className="absolute inset-0 bg-gradient-to-b from-black/30 via-black/10 to-black/40" />
        </div>
        
        {/* Background Elements */}
        <div className="absolute top-20 left-10 w-72 h-72 bg-blue-400/20 rounded-full blur-3xl" />
        <div className="absolute bottom-20 right-10 w-96 h-96 bg-blue-400/20 rounded-full blur-3xl" />
        
        <div className="relative max-w-7xl mx-auto">
          <div className="text-center">
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8 }}
              className="mb-12"
            >
              <h1 className="text-4xl md:text-8xl font-black text-white mb-8 leading-tight">
                Seamless{' '}
                <span className="primary-gradient bg-clip-text text-transparent">
                  Web3
                </span>{' '}
                payments for everyone
          </h1>

                          <p className="text-xl md:text-2xl text-white/90 max-w-4xl mx-auto leading-relaxed font-medium">
                            Create beautiful payment pages, accept crypto globally, and grow your community
                            with the most intuitive Web3 payment platform.
                          </p>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: 0.4 }}
              className="flex flex-col sm:flex-row gap-6 justify-center items-center mb-20"
            >
              <Link href="/role-selection">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="btn-primary text-lg px-8 py-4 flex items-center space-x-3"
                >
                  <span>Create Your Page</span>
                  <ArrowRight className="w-5 h-5" />
                </motion.button>
              </Link>
              <Link href="/public-page">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="btn-secondary text-lg px-8 py-4"
                >
                  Explore Creators
                </motion.button>
              </Link>
            </motion.div>

            {/* Role Cards */}
            <motion.div
              initial={{ opacity: 0, y: 30 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8, delay: 0.6 }}
              className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto"
            >
              {[
                {
                  title: "Creator",
                  description: "Monetize your content and build a loyal community",
                  icon: <Users className="w-8 h-8" />,
                  color: "from-blue-400 to-blue-600",
                  href: "/role-selection"
                },
                {
                  title: "Business",
                  description: "Accept payments and sell products globally",
                  icon: <Building2 className="w-8 h-8" />,
                  color: "from-blue-500 to-cyan-500",
                  href: "/role-selection"
                },
                {
                  title: "Crowdfunder",
                  description: "Launch campaigns and reach your funding goals",
                  icon: <Target className="w-8 h-8" />,
                  color: "from-blue-600 to-blue-800",
                  href: "/role-selection"
                }
              ].map((role, index) => (
                <Link key={role.title} href={role.href}>
                  <motion.div
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.5, delay: 0.8 + index * 0.1 }}
                    whileHover={{ scale: 1.05, y: -8 }}
                    className="glass-card rounded-2xl p-8 card-hover cursor-pointer"
                  >
                    <div className={`w-16 h-16 bg-gradient-to-br ${role.color} rounded-2xl flex items-center justify-center text-white mb-6`}>
                      {role.icon}
                    </div>
                    <h3 className="text-2xl font-bold text-slate-900 mb-3">
                      {role.title}
                    </h3>
                    <p className="text-slate-600 leading-relaxed">
                      {role.description}
                    </p>
                  </motion.div>
                </Link>
              ))}
            </motion.div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-24 bg-white/50 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-20">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.8 }}
            >
              <h2 className="text-5xl md:text-6xl font-black text-slate-900 mb-6">
                Why choose OnClick?
              </h2>
              <p className="text-xl text-slate-600 max-w-3xl mx-auto font-medium">
                Built for creators, businesses, and dreamers who want to focus on what matters most
              </p>
            </motion.div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {[
              {
                title: "Lightning Fast",
                description: "Payments processed in seconds, not minutes. Your supporters get instant confirmation.",
                icon: <Zap className="w-8 h-8" />
              },
              {
                title: "Global Reach",
                description: "Accept payments from anywhere in the world. No borders, no restrictions.",
                icon: <Globe className="w-8 h-8" />
              },
              {
                title: "Bank-Grade Security",
                description: "Your funds are protected with enterprise-level security and decentralization.",
                icon: <Shield className="w-8 h-8" />
              },
              {
                title: "Multiple Payment Options",
                description: "Crypto, cards, bank transfers. Your supporters choose how they want to pay.",
                icon: <CheckCircle className="w-8 h-8" />
              },
              {
                title: "Beautiful Pages",
                description: "Create stunning payment pages that reflect your brand and personality.",
                icon: <Sparkles className="w-8 h-8" />
              },
              {
                title: "Real-time Analytics",
                description: "Track your success with detailed insights and supporter engagement metrics.",
                icon: <Users className="w-8 h-8" />
              }
            ].map((feature, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: index * 0.1 }}
                whileHover={{ scale: 1.02 }}
                className="bg-white/80 backdrop-blur-sm rounded-2xl p-8 card-hover border border-white/20"
              >
                <div className="w-16 h-16 primary-gradient rounded-2xl flex items-center justify-center text-white mb-6">
                  {feature.icon}
                </div>
                <h3 className="text-2xl font-bold text-slate-900 mb-4">
                  {feature.title}
                </h3>
                <p className="text-slate-600 leading-relaxed">
                  {feature.description}
                </p>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

                  {/* CTA Section */}
                 <section className="py-24 relative overflow-hidden">
                    <div className="absolute inset-0 bg-gradient-to-r from-blue-600 to-blue-800" />
                    <div className="absolute inset-0 bg-black/20" />
        
        <div className="relative max-w-4xl mx-auto text-center px-4 sm:px-6 lg:px-8">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
          >
            <h2 className="text-5xl md:text-6xl font-black text-white mb-6">
              Ready to get started?
            </h2>
            <p className="text-xl text-white/90 mb-12 font-medium">
              Join thousands of creators who are already building their Web3 presence
            </p>
            <Link href="/role-selection">
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                className="bg-white text-blue-600 text-xl font-bold px-12 py-6 rounded-2xl hover:bg-gray-50 transition-all duration-300 shadow-2xl"
              >
                Create Your Page Now
              </motion.button>
            </Link>
          </motion.div>
        </div>
      </section>

      <Footer />
    </div>
  );
}
