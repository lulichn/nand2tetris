package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.w3c.dom.Node;

import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.OutputKeys;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import java.io.*;
import java.nio.file.Paths;

public class Main {
  public static void main(String[] args) throws IOException, ParserConfigurationException, TransformerException {
    var path = Paths.get(args[0]);
    var inFile = path.toFile();

    if (inFile.isFile()) {
      compile(inFile);
    } else if (inFile.isDirectory()) {
      compileDir(inFile);
    }
  }

  public static void compileDir(File directory) throws IOException, ParserConfigurationException, TransformerException {
    var children = directory.listFiles((dir, name) -> name.toLowerCase().endsWith(".jack"));
    for (var file: children) {
      compile(file);
    }
  }

  public static void compile(File file) throws IOException, ParserConfigurationException, TransformerException {
    Node node;

    try (var is = new FileInputStream(file)) {
      var lexer = new JackLexer(CharStreams.fromStream(is));
      var stream = new CommonTokenStream(lexer);
      var parser = new JackParser(stream);
      node = ToXml.klass(parser.klass());
    }


    var name = getNameWithoutExtension(file.getCanonicalPath());

    try (var os = new FileOutputStream(new File(name + ".xml"))) {
      outputXml(node, os);
    }
  }

  public static String getNameWithoutExtension(String fileName) {
    int index = fileName.lastIndexOf('.');
    if (index!=-1) {
      return fileName.substring(0, index);
    }

    return fileName;
  }

  public static void outputXml(Node document, OutputStream os) throws TransformerException {
    TransformerFactory factory = TransformerFactory.newInstance();
    Transformer transformer = factory.newTransformer();

    transformer.setOutputProperty(OutputKeys.INDENT, "yes");
    transformer.setOutputProperty(OutputKeys.METHOD, "html");
    transformer.setOutputProperty("{http://xml.apache.org/xalan}indent-amount", "2");

    transformer.transform(new DOMSource(document), new StreamResult(os));
  }
}
