(ns eval-exec.day6
  (:require [clojure.string :as str])) ; Add this line to require clojure.string

(defn parse-input
  [filename]
  (let [lines (slurp filename)]
    (map #(str/split % #",") (str/split lines #"\n")))) ; Use str/split instead of clojure.string/split

(defn -main
  "I don't do a whole lot ... yet."
  []
  (println
   (parse-input "input.txt")))
