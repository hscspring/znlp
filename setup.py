import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="znlp",
    version="0.0",
    author="Yam",
    author_email="haoshaochun@gmail.com",
    description="中文 NLP.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/hscspring/znlp",
    packages=setuptools.find_packages(),
    install_requires=[
      ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)