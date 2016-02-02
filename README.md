The **llamapun** library aims to provide support for a wide array of common **language and mathematics processing** tasks.

[![Build Status](https://secure.travis-ci.org/KWARC/llamapun.png?branch=master)](http://travis-ci.org/KWARC/llamapun)

---
The migration consists of reorganizing the libraries, and preparing a CPAN-near bundle including a testbed and detailed documentation.
This process also brings a namespace change to the now properly spelled LLaMaPUn.

Several upcoming deployments of the [CorTeX framework](https://github.com/dginev/CorTeX) have motivated the move to GitHub
and provide an outlook for a number of fixes and features to be added to the library.

## High-level Overview
 * **Source Data**
   * Built-in support for STEM documents in ([LaTeXML-flavoured](https://github.com/brucemiller/LaTeXML/)) HTML5.
 * **Preprocessing**
   * Unicode normalization,
   * Stopwords - based on widely accepted lists, enhanced for STEM texts,
   * Semi-structured to plain text normalization (math, citations, tables, etc.),
   * Purification of text and math modality (e.g. move trailing dots left in math back into the sentence text),
   * Stemming - adaptation of the [Morpha](http://www.sussex.ac.uk/Users/johnca/morph.html) stemmer,
   * Tokenization - rule-based sentence segmentation, and [SENNA](http://ml.nec-labs.com/senna/) word tokenization
 
 * **Shallow Analysis**
   * Language identification (via [libTextCat](http://software.wise-guys.nl/libtextcat/)),
   * N-gram footprints,
   * Part-of-speech tagging (via [SENNA](http://ml.nec-labs.com/senna/)),
   * Named Entity recognition (via [SENNA](http://ml.nec-labs.com/senna/)),
   * Chunking and shallow parsing (via [SENNA](http://ml.nec-labs.com/senna/)),
   * [TODO] "Definition" paragraph discrimination task (training SVM classifiers, based on TF/IDF and Ngram BoW features, via [libsvm](https://github.com/cjlin1/libsvm))
   * [TODO] "Declaration" sentence discrimination task (training CRF models via [CRFsuite](http://www.chokkan.org/software/crfsuite/)).
 
 * **Representation Toolkit**
   * Document Narrative Model (DNM) addition to the XML DOM
   * XPointer and string offset annotation support
   * Integration with the [CorTeX](https://github.com/dginev/CorTeX) processing framework
   * [TOPORT] Shared Packed parse forests for mathematical formulas (aka "disjunctive logical forms")

 * **Programming API**
   * High-level iterators over the narrative elements of scientific documents
   * Zero-cost abstractions over the source data, as well as over linguistic annotations of various granularity.

 
## Contact
Feel free to send any feedback to the project maintainer at d.ginev@jacobs-university.de

---

Please remember that all third-party tools (such as the [SENNA](http://ml.nec-labs.com/senna/) NLP toolkit) enforce their own licensing constraints.
**Disclaimer:** This Github repository is a successor to the now deprecated [C+Perl LLaMaPUn implementation](https://github.com/KWARC/deprecated-LLaMaPUn).
