package Modelo;

import java.util.LinkedList;
import java.util.List;

	/**

	*Classe Estoque que armazena informações sobre o estoque, incluindo a disponibilidade de empilhadeiras,
	*a quantidade de caminhões dentro do estoque e a lista de itens presentes no estoque.
	*Possui variáveis de instância públicas para armazenar a disponibilidade de empilhadeiras e a quantidade de
	*caminhões dentro do estoque.
	*Agrega a classe Item por meio de uma LinkedList para armazenar os itens presentes no estoque.
	*Possui construtores para inicializar os atributos, métodos de acesso e modificação para cada atributo,
	*e um método para adicionar um item à lista de itens do estoque.
	
	* @author Carlos Henrique
	* @author André João
	* @author Sunamita Vitória
	* 
	* @version 2.0	
	*/

public class Estoque {
	
	
	private int empilhadeiradisponivel;
	private int caminhaoDentro;
	
	/**
	 * LinkedList da classe Item que representa a agregação de dados da classe Estoque.
	 */
	
	private List<Item> itens = new LinkedList<Item>();

	/**
	 * Construtor da classe Estoque que recebe todas as informações necessárias, incluindo a lista de itens.
	 * 
	 * @param empilhadeiradisponivel a disponibilidade de empilhadeiras no estoque
	 * @param caminhaoDentro         a quantidade de caminhões dentro do estoque
	 * @param itens                  a lista de itens presentes no estoque
	 */
	
	public Estoque(int empilhadeiradisponivel, int caminhaoDentro, List<Item> itens) {
		super();
		this.empilhadeiradisponivel = empilhadeiradisponivel;
		this.caminhaoDentro = caminhaoDentro;
		this.itens = itens;
	}
	
	/**
	 * Construtor da classe Estoque que recebe apenas as informações básicas do estoque.
	 * 
	 * @param empilhadeiradisponivel a disponibilidade de empilhadeiras no estoque
	 * @param caminhaoDentro         a quantidade de caminhões dentro do estoque
	 */
	
	public Estoque(int empilhadeiradisponivel, int caminhaoDentro) {
		super();
		this.empilhadeiradisponivel = empilhadeiradisponivel;
		this.caminhaoDentro = caminhaoDentro;
		
	}

	/**
	 * Construtor da classe Estoque que recebe apenas as informações básicas do estoque.
	 * 
	 * @param empilhadeiradisponivel a disponibilidade de empilhadeiras no estoque
	 * @param caminhaoDentro         a quantidade de caminhões dentro do estoque
	 */
	
	public Estoque() {
		
	}

	public int getEmpilhadeiradisponivel() {
		return empilhadeiradisponivel;
	}
	
	/**
	 * Método de modificação para definir a disponibilidade de empilhadeiras no estoque.
	 * 
	 * @param empilhadeiradisponivel a disponibilidade de empilhadeiras no estoque
	 */

	public void setEmpilhadeiradisponivel(int empilhadeiradisponivel) {
		this.empilhadeiradisponivel = empilhadeiradisponivel;
	}
	
	/**
	 * Método de acesso para obter a quantidade de caminhões dentro do estoque.
	 * 
	 * @return a quantidade de caminhões dentro do estoque
	 */

	public int getCaminhaoDentro() {
		return caminhaoDentro;
	}
	
	/**
	 * Método de modificação para definir a quantidade de caminhões dentro do estoque.
	 * 
	 * @param caminhaoDentro a quantidade de caminhões dentro do estoque
	 */

	public void setCaminhaoDentro(int caminhaoDentro) {
		this.caminhaoDentro = caminhaoDentro;
	}
	
	/**
	 * Método de acesso para obter a lista de itens presentes no estoque.
	 * 
	 * @return a lista de itens presentes no estoque
	 */

	public List<Item> getItens() {
		return itens;
	}
	
	/**
	 * Método de acesso para obter a lista de itens presentes no estoque.
	 * 
	 * @return a lista de itens presentes no estoque
	 */

	public void setItens(List<Item> itens) {
		this.itens = itens;
	}
	
	/**
	 * Método para adicionar um novo item à lista de itens do estoque.
	 * 
	 * @param item o novo item a ser adicionado
	 */

	/**
	 * Sobrescrita do método toString() para exibir as informações do estoque.
	 * 
	 * @return uma string vazia (método não utilizado no momento)
	 */
	
	public String toString() {
		return "";
	}
	
}
