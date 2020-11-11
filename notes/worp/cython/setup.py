from distutils.core import Extension, setup
from Cython.Build import cythonize

#works!!!!  except weird rbenv error using "build"... doesn't matter???
# https://riptutorial.com/cython/example/14478/hello-world

# define an extension that will be cythonized and compiled
ext = Extension(name="hello", sources=["hello.pyx"])
setup(ext_modules=cythonize(ext))