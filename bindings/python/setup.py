
from setuptools import setup
from setuptools_rust import Binding, RustExtension

extras = {}

setup(
    name='sorted',
    version='0.1.0',
    description='Collection of sorting algorithms.',
    long_description=open('README.md', 'r', encoding='utf-8').read(),
    long_description_content_type='text/markdown',
    keywords=['rust', 'sort', 'algorithms'],
    author='Victor I. Afolabi',
    author_email='javafolabi@gmail.com',
    url='https://github.com/victor-yi/sorted',
    license='Apache License 2.0',
    rust_extensions=[RustExtension('sorted.sorted',
                                   binding=Binding.PyO3,
                                   debug=False)],
    extras_require=extras,
    classifiers=[
        'Development Status :: 5 - Production/Stable',
        'Intended Audience :: Developers',
        'Intended Audience :: Education',
        'Intended Audience :: Science/Research',
        'License :: OSI Approved :: Apache Software License',
        'Operating System :: POSIX',
        'Operating System :: OS Independent',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
    ],
    package_dir={'': 'py_src'},
    packages=[
        'sorted',
    ],
    # package_data={
    #     'sorted': ['py.typed', '__init__.pyi'],
    # },
    zip_safe=False,
)
