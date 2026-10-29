import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';

export default function CollectionsPageFa() {
  const meta = {
    title: 'آرایه‌ها، مپ‌ها و دیکشنری',
    category: 'هسته زبان',
    description: 'راهنمای ساختارهای داده کالکشن در پرسیکا: آرایه‌های تایپ‌شده، ماتریس‌های چندبعدی، توابع کمکی توکار، دیکشنری داینامیک و مپ‌های تایپ‌شده.',
  };

  const toc = [
    { title: 'آرایه‌های تایپ‌شده', id: 'arrays', level: 2 },
    { title: 'آرایه‌های چندبعدی و ماتریس', id: 'multidimensional', level: 2 },
    { title: 'توابع کمکی آرایه', id: 'helpers', level: 2 },
    { title: 'دیکشنری‌های داینامیک (dict)', id: 'dict', level: 2 },
    { title: 'مپ‌های تایپ‌شده (map)', id: 'maps', level: 2 },
    { title: 'کلیدها، مقادیر و توابع بازرسی', id: 'inspection', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="arrays" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        آرایه‌های تایپ‌شده
      </h2>
      <p>
        آرایه‌ها در پرسیکا از اندیس صفر شروع می‌شوند، به طور پویا رشد می‌کنند و دارای نوع مشخصی برای تضمین همگنی عناصر هستند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="arrays.persi"
          code={`// آرایه‌ای از اعداد صحیح
int[] numbers = [10, 20, 30, 40];

// دسترسی و تغییر مقدار با اندیس
print("First element:", numbers[0]);
numbers[1] = 99; // تغییر مقدار خانه دوم

// اندیس منفی برای دسترسی از انتهای آرایه
print("Last element via -1:", numbers[-1]);
print("Updated array:", numbers);`}
          output={`First element: 10
Last element via -1: 40
Updated array: [10, 99, 30, 40]`}
        />
      </div>

      <h2 id="multidimensional" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        آرایه‌های چندبعدی و ماتریس
      </h2>
      <p>
        آرایه‌های تو در تو به راحتی با تکرار جفت کروشه‌ها (مانند <code className="text-emerald-400">int[][]</code>) تعریف می‌شوند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="matrix.persi"
          code={`int[][] matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

print("Center element [1][1]:", matrix[1][1]);
matrix[0][2] = 99;
print("Row 0 updated:", matrix[0]);`}
          output={`Center element [1][1]: 5
Row 0 updated: [1, 2, 99]`}
        />
      </div>

      <h2 id="helpers" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        توابع کمکی آرایه
      </h2>
      <div className="overflow-x-auto my-4 text-xs">
        <table className="w-full text-right border border-slate-800 rounded-xl overflow-hidden">
          <thead className="bg-slate-900/80 text-slate-300 font-bold border-b border-slate-800">
            <tr>
              <th className="p-3">تابع</th>
              <th className="p-3">خروجی</th>
              <th className="p-3">توضیحات</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800 text-slate-400">
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">len(arr)</td>
              <td className="p-3 font-mono" dir="ltr">int</td>
              <td className="p-3">تعداد عناصر آرایه را برمی‌گرداند.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">append(arr, item)</td>
              <td className="p-3 font-mono" dir="ltr">void</td>
              <td className="p-3">عنصر جدید را به انتهای آرایه اضافه می‌کند.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">sort(arr)</td>
              <td className="p-3 font-mono" dir="ltr">array</td>
              <td className="p-3">یک کپی مرتب‌شده به صورت صعودی برمی‌گرداند.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">reverse(arr)</td>
              <td className="p-3 font-mono" dir="ltr">array</td>
              <td className="p-3">یک کپی معکوس از آرایه برمی‌گرداند.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">slice(arr, start, end)</td>
              <td className="p-3 font-mono" dir="ltr">array</td>
              <td className="p-3">برش زیرمجموعه از آرایه با پشتیبانی از اندیس‌های منفی.</td>
            </tr>
            <tr>
              <td className="p-3 font-mono text-emerald-400" dir="ltr">sum(arr)</td>
              <td className="p-3 font-mono" dir="ltr">int | float</td>
              <td className="p-3">مجموع تمامی عناصر عددی آرایه.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="helpers_demo.persi"
          code={`int[] data = [42, 10, 88, 3, 27];

print("Original:", data);
print("Sorted copy:", sort(data));
print("Reversed copy:", reverse(data));
print("Sum of elements:", sum(data));
print("Slice [1:4]:", slice(data, 1, 4));`}
          output={`Original: [42, 10, 88, 3, 27]
Sorted copy: [3, 10, 27, 42, 88]
Reversed copy: [27, 3, 88, 10, 42]
Sum of elements: 170
Slice [1:4]: [10, 88, 3]`}
        />
      </div>

      <h2 id="dict" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        دیکشنری‌های داینامیک (`dict`)
      </h2>
      <p>
        برای داده‌های ساختاریافته کلید-مقدار شبیه JSON که مقادیر می‌توانند از انواع گوناگون باشند، از <code className="text-emerald-400">dict</code> استفاده می‌شود:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="dictionaries.persi"
          code={`dict user = {
    "id": 101,
    "name": "محمد طاها",
    "verified": true,
    "rating": 4.9
};

print("User Name:", user["name"]);
user["role"] = "Admin";
print("User updated:", user);`}
          output={`User Name: محمد طاها
User updated: {"id": 101, "name": "محمد طاها", "verified": true, "rating": 4.9, "role": "Admin"}`}
        />
      </div>

      <h2 id="maps" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        مپ‌های تایپ‌شده (map&lt;K, V&gt;)
      </h2>
      <p>
        زمانی که می‌خواهید نوع کلید و مقدار به شکل ایستا در زمان کامپایل بررسی شود، از ساختار <code className="text-emerald-400">map&lt;KeyType, ValueType&gt;</code> بهره ببرید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="typed_map.persi"
          code={`map<string, int> scores = {
    "Alice": 95,
    "Bob": 88
};

scores["Charlie"] = 92;
print("Alice's score:", scores["Alice"]);`}
          output={`Alice's score: 95`}
        />
      </div>

      <h2 id="inspection" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        کلیدها، مقادیر و توابع بازرسی
      </h2>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="inspection.persi"
          code={`dict config = {"host": "127.0.0.1", "port": 8080};

if contains_key(config, "port") {
    print("Port is configured:", config["port"]);
}

print("All keys:", keys(config));
print("All values:", values(config));`}
          output={`Port is configured: 8080
All keys: ["host", "port"]
All values: ["127.0.0.1", 8080]`}
        />
      </div>
    </DocsLayout>
  );
}
