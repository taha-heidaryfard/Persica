import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { Terminal, Shield, Command } from 'lucide-react';

export default function CliReferencePageFa() {
  const meta = {
    title: 'مرجع جامع دستورات خط فرمان (CLI)',
    category: 'ابزارها و اکوسیستم',
    description: 'راهنمای کامل تمامی آرگومان‌ها، زیردستورها، فلگ‌ها و حالات تشخیصی در خط فرمان کامپایلر پرسیکا.',
  };

  const toc = [
    { title: 'نمای کلی دستورات', id: 'overview', level: 2 },
    { title: 'زیردستورهای مدیریت پروژه', id: 'project-commands', level: 2 },
    { title: 'حالات اجرای مستقیم', id: 'execution-modes', level: 2 },
    { title: 'تحلیل استاتیک و بازرسی کد', id: 'inspection-flags', level: 2 },
    { title: 'ورودی استاندارد (Pipelines)', id: 'stdin', level: 2 },
    { title: 'کدهای خروجی و گزارش خطاها', id: 'exit-codes', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="overview" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        نمای کلی دستورات
      </h2>
      <p>
        فایل اجرایی <code className="text-emerald-400 font-mono">persica</code> یک ابزار باینری یکپارچه و مستقل است که تمامی امور کامپایل، تفسیر، ساخت پروژه، تست خودکار و بررسی نوع را به عهده دارد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`persica [SUBCOMMAND] [OPTIONS] [FILE]`}
        />
      </div>

      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-right border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">دستور</th>
              <th className="p-3">شرح عملکرد</th>
              <th className="p-3">نمونه استفاده</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica &lt;file.persi&gt;</td>
              <td className="p-3">اجرای مستقیم یک اسکریپت تکی</td>
              <td className="p-3 font-mono" dir="ltr">persica app.persi</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica new &lt;name&gt;</td>
              <td className="p-3">ایجاد داربست‌بندی کامل یک پروژه جدید</td>
              <td className="p-3 font-mono" dir="ltr">persica new web_api</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica init</td>
              <td className="p-3">تبدیل پوشه جاری به یک پروژه پرسیکا</td>
              <td className="p-3 font-mono" dir="ltr">persica init</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica run</td>
              <td className="p-3">اجرای فایل ورودی تعریف‌شده در persica.toml</td>
              <td className="p-3 font-mono" dir="ltr">persica run</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica test</td>
              <td className="p-3">اجرای تمامی تست‌های پوشه tests/</td>
              <td className="p-3 font-mono" dir="ltr">persica test</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica check &lt;file&gt;</td>
              <td className="p-3">بررسی سینتکس و انواع بدون اجرا</td>
              <td className="p-3 font-mono" dir="ltr">persica check src/main.persi</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica --eval "&lt;code&gt;"</td>
              <td className="p-3">ارزیابی یک خط کد تستی در ترمینال</td>
              <td className="p-3 font-mono" dir="ltr">persica --eval "print(2+2);"</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">persica -</td>
              <td className="p-3">خواندن کدها از ورودی استاندارد (پایپ)</td>
              <td className="p-3 font-mono" dir="ltr">cat main.persi | persica -</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2 id="project-commands" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        زیردستورهای مدیریت پروژه
      </h2>

      <div className="space-y-6 my-4">
        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica new &lt;name&gt;</code></h4>
          <p className="text-xs text-slate-400 mb-2">یک پوشه جدید همراه با فایل کانفیگ <code className="text-slate-300">persica.toml</code>، کد ورودی <code className="text-slate-300">src/main.persi</code> و پوشه پکیج‌های ایزوله <code className="text-slate-300">.persi_env/</code> ایجاد می‌کند.</p>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica run</code></h4>
          <p className="text-xs text-slate-400 mb-2">به طور خودکار فایل <code className="text-slate-300">persica.toml</code> را در پوشه جاری یا پوشه‌های والد جستجو کرده، مسیر ماژول‌ها را تنظیم و پروژه را اجرا می‌نماید.</p>
        </div>

        <div>
          <h4 className="text-sm font-bold text-white mb-1"><code className="text-emerald-400 font-mono">persica test</code></h4>
          <p className="text-xs text-slate-400 mb-2">تمام فایل‌های تستی <code className="text-slate-300">*.persi</code> در پوشه <code className="text-slate-300">tests/</code> را اجرا کرده و گزارش قبولی/ردی به همراه کد خروجی برای CI ارائه می‌دهد.</p>
        </div>
      </div>

      <h2 id="inspection-flags" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        تحلیل استاتیک و بازرسی کد
      </h2>
      <p>
        پرسیکا فلگ‌های تشخیصی متعددی برای ادغام با ادیتورها و افزونه‌های IDE دارد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`# ۱. بررسی نوع بدون اجرا
persica check main.persi
# Output: "check succeeded" (exit code 0)

# ۲. خروجی توکن‌های لکسر
persica --tokens main.persi

# ۳. خروجی درخت نحوی انتزاعی (AST)
persica --ast main.persi`}
        />
      </div>

      <h2 id="stdin" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ورودی استاندارد (Pipelines)
      </h2>
      <p>
        با ارسال کاراکتر خط فاصله <code className="text-emerald-400 font-bold">-</code> کدها مستقیماً از طریق پایپینگ شل اجرا می‌شوند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="bash"
          filename="Terminal"
          code={`echo 'print("Piped execution!");' | persica -`}
          output={`Piped execution!`}
        />
      </div>

      <h2 id="exit-codes" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        کدهای خروجی و گزارش خطاها
      </h2>
      <p>
        پرسیکا با کدهای استاندارد وضعیت با اسکریپت‌ها و پایپ‌لاین‌های CI ارتباط برقرار می‌کند:
      </p>
      <ul className="text-xs text-slate-300 list-disc pr-5 space-y-1 my-3">
        <li><code className="text-emerald-400">0</code> — موفقیت‌آمیز (اجرا یا بررسی نوع بدون نقص به پایان رسید).</li>
        <li><code className="text-rose-400">1</code> — خطای کامپایل یا رانتایم. جزئیات دقیق خطا با شماره سطر و ستون در <code className="text-slate-300">stderr</code> چاپ می‌شود: <code className="text-slate-300">error[phase]: message --&gt; file:line:col</code>.</li>
      </ul>
    </DocsLayout>
  );
}
