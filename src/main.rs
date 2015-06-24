extern crate llamapun;
extern crate rustlibxml;
extern crate libc;
extern crate gnuplot;

use llamapun::dnmlib::*;
use llamapun::tokenizer::*;
use rustlibxml::tree::*;
use rustlibxml::xpath::*;
use std::collections::HashMap;
use gnuplot::*;

fn main() {

  let doc = XmlDoc::parse_html_file("tests/resources/0903.1000.html").unwrap();
  let mut dictionary: HashMap<String, i64> = HashMap::new();
  let mut frequencies: HashMap<String, i64> = HashMap::new();
  let mut word_index = 0;

  // We will tokenize each logical paragraph, which are the textual logical units in an article
  let xpath_context = XmlXPathContext::new(&doc).unwrap();
  let para_xpath_result = xpath_context.evaluate("//*[contains(@class,'ltx_para')]").unwrap();

  for para in para_xpath_result.get_nodes_as_vec().iter() {
    let mut dnm_options : HashMap<String, SpecialTagsOption> = HashMap::new();
    dnm_options.insert("math".to_string(), SpecialTagsOption::Normalize("MathFormula".to_string()));

    let dnm = DNM::create_dnm(&para,
      DNMParameters {special_tag_name_options : dnm_options,
                     wrap_tokens : false,
                     normalize_white_spaces : false,
                     special_tag_class_options : HashMap::new(),
                     move_whitespaces_between_nodes: false,
                     normalize_unicode: true,  
                    });

    let tokenizer = Tokenizer::default();
    let ranges : Vec<DNMRange> = tokenizer.sentences(&dnm).unwrap();

    for range in ranges {
      let sentence = range.get_plaintext();
      for w in sentence.split(|c: char| !c.is_alphabetic()) {
        if w.len() == 0 {
          continue;
        }
        let word = w.to_string().to_lowercase();
        let dictionary_index : &i64 = 
          match dictionary.contains_key(&word) {
          true => dictionary.get(&word).unwrap(),
          false => {
            word_index+=1;
            dictionary.insert(word.clone(), word_index);
            &word_index }
          };
        print!("{}  ",dictionary_index);
        let word_frequency = frequencies.entry(word).or_insert(0);
        *word_frequency += 1;
      }
      println!("");
    }
  }
  println!("");

  let mut sorted_dictionary = Vec::new();
  for (word, index) in dictionary.iter() {
    sorted_dictionary.push((word,index));
  }
  

  let mut sorted_frequencies = Vec::new();
  for (word, index) in frequencies.iter() {
    sorted_frequencies.push((word,index));
  }

  // Unsorted gnuplot of frequencies:
  let dict_indices = sorted_dictionary.clone().into_iter().map(|entry| entry.1);
  let freq_values = sorted_frequencies.clone().into_iter().map(|entry| entry.1);
  let mut fg = Figure::new();
  fg.axes2d()
  .points(dict_indices, freq_values, &[PointSymbol('D'), Color("#ffaa77"), PointSize(1.5)])
  .set_x_label("Words, in order of appearance", &[Rotate(45.0)])
  .set_y_label("Frequency counts", &[Rotate(90.0)])
  .set_title("Example Word Frequencies", &[]);

  fg.set_terminal("pngcairo", "word_frequnecies_inorder.png");
  fg.show();
  
  // Sorted gnuplot of frequencies:
  // Perform sort
  sorted_dictionary.sort_by(|a, b| a.1.cmp(b.1));
  sorted_frequencies.sort_by(|a, b| a.1.cmp(b.1));
  let sorted_dict_indices = sorted_dictionary.clone().into_iter().map(|entry| entry.1);
  let sorted_freq_values = sorted_frequencies.clone().into_iter().map(|entry| entry.1);

  fg = Figure::new();
  fg.axes2d()
  .points(sorted_dict_indices, sorted_freq_values, &[PointSymbol('D'), Color("#ffaa77"), PointSize(1.5)])
  .set_x_label("Words, in order of appearance", &[Rotate(45.0)])
  .set_y_label("Frequency counts", &[Rotate(90.0)])
  .set_title("Example Word Frequencies", &[]);

  fg.set_terminal("pngcairo", "word_frequnecies_sorted.png");
  fg.show();

  // Print out data:
  println!("Dictionary: \n{:?}\n\n", sorted_dictionary);
  println!("Frequencies: \n{:?}\n\n", sorted_frequencies);

  




}

