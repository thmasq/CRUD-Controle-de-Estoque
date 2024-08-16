


package Controle;

import java.util.ArrayList;

import Modelo.Eletronico;
import Modelo.Movel;

/**

	*Classe de controle de estoque.
*Esta classe é responsável por controlar o estoque de móveis, eletrônicos e itens em geral.
*Ela possui métodos para adicionar e recuperar informações sobre os diferentes tipos de produtos.
*Os produtos são armazenados em listas separadas (moveis, eletronicos e item).
*As listas de produtos são do tipo ArrayList para permitir a adição dinâmica de elementos.
*Para cada tipo de produto (móveis, eletrônicos e itens), existe um método para salvar o produto na lista
*e um método correspondente para retornar a lista de produtos desse tipo.
*Os métodos de salvar retornam um valor booleano indicando se a operação foi bem sucedida (true) ou não (false).
*Um produto só será salvo se for diferente de null.

	* @author Carlos Henrique
	* @author André João
	* @author Sunamita Vitória
	* 
	* @version 2.0	

 */

public class ControleEstoque {

	
	
	private ArrayList<Movel> moveis = new ArrayList <Movel>();
	
	/**
	 * Salva um objeto Movel na lista de móveis.
	 * 
	 * @param m o objeto Movel a ser salvo
	 * @return true se o objeto for salvo com sucesso, false caso contrário
	 */
	
	public boolean salvarmoveis (Movel m) {
		if (m != null) {
			moveis.add(m);
			return true;
		}else {
			return false;
		}
	}
	
	/**
	 * Retorna a lista de móveis.
	 * 
	 * @return a lista de móveis
	 */
	
	public ArrayList<Movel> retornamoveis (){
		return moveis;
	}
	
	private ArrayList<Eletronico> eletronicos = new ArrayList <Eletronico>();
	
	/**
	 * Salva um objeto Eletronico na lista de eletrônicos.
	 * 
	 * @param e o objeto Eletronico a ser salvo
	 * @return true se o objeto for salvo com sucesso, false caso contrário
	 */
	
	public boolean salvareletronicos (Eletronico e) {
		if (e != null) {
			eletronicos.add(e);
			return true;
		}else {
			return false;
		}
	}
	
	/**
	 * Retorna a lista de eletrônicos.
	 * 
	 * @return a lista de eletrônicos
	 */
	
	public ArrayList<Eletronico> retornaeletronicos (){
		return eletronicos;
	}
	
}
