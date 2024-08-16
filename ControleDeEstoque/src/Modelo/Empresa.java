package Modelo;

import java.util.LinkedList;
import java.util.List;

	/**

	*Classe Empresa que representa informações sobre uma empresa.
	*Essa classe armazena dados como nome, endereço e CNPJ da empresa.
	*Também possui uma lista de filiais associadas à empresa.
	*Possui construtores para inicializar os atributos e métodos de acesso e modificação para cada atributo.
	*Além disso, contém um método para adicionar uma nova filial à lista de filiais.
	
 	* @author Carlos Henrique
	* @author André João
	* @author Sunamita Vitória
	* 
	* @version 2.0	
	*/

public class Empresa {
	

	private String nome;
	private String endereço;
	private double cnpj;

	/**
	 * LinkedList de Filial que representa a agregação da classe Empresa.
	 */

	private List<Filial> filiais = new LinkedList<Filial>();

	/**
	 * Construtor da classe Empresa que recebe todas as informações necessárias, incluindo a lista de filiais.
	 * 
	 * @param nome     o nome da empresa
	 * @param endereço o endereço da empresa
	 * @param cnpj     o CNPJ da empresa
	 * @param filiais  a lista de filiais associadas à empresa
	 */

	public Empresa(String nome, String endereço, double cnpj, List<Filial> filiais) {
		this.nome = nome;
		this.endereço = endereço;
		this.cnpj = cnpj;
		this.filiais = filiais;
	}

	/**
	 * Construtor da classe Empresa que recebe apenas as informações básicas da empresa.
	 * 
	 * @param nome     o nome da empresa
	 * @param endereço o endereço da empresa
	 * @param cnpj     o CNPJ da empresa
	 */

	public Empresa(String nome, String endereço, double cnpj) {
		this.nome = nome;
		this.endereço = endereço;
		this.cnpj = cnpj;
	}

	/**
	 * Método de acesso para obter o nome da empresa.
	 * 
	 * @return o nome da empresa
	 */
	public String getNome() {
		return nome;
	}

	/**
	 * Método de modificação para definir o nome da empresa.
	 * 
	 * @param nome o nome da empresa
	 */
	public void setNome(String nome) {
		this.nome = nome;
	}

	/**
	 * Método de acesso para obter o endereço da empresa.
	 * 
	 * @return o endereço da empresa
	 */
	public String getEndereço() {
		return endereço;
	}

	/**
	 * Método de modificação para definir o endereço da empresa.
	 * 
	 * @param endereço o endereço da empresa
	 */
	public void setEndereço(String endereço) {
		this.endereço = endereço;
	}

	/**
	 * Método de acesso para obter o CNPJ da empresa.
	 * 
	 * @return o CNPJ da empresa
	 */
	public double getCnpj() {
		return cnpj;
	}

	/**
	 * Método de modificação para definir o CNPJ da empresa.
	 * 
	 * @param cnpj o CNPJ da empresa
	 */
	public void setCnpj(double cnpj) {
		this.cnpj = cnpj;
	}

	/**
	 * Método de acesso para obter a lista de filiais associadas à empresa.
	 * 
	 * @return a lista de filiais associadas à empresa
	 */
	public List<Filial> getFiliais() {
		return filiais;
	}

	/**
	 * Método de modificação para definir a lista de filiais associadas à empresa.
	 * 
	 * @param filiais a lista de filiais associadas à empresa
	 */
	public void setFiliais(List<Filial> filiais) {
		this.filiais = filiais;
	}

	/**
	 * Método para adicionar uma nova filial à lista de filiais da empresa.
	 * 
	 * @param filial a nova filial a ser adicionada
	 */
	public void adicionaFilial(Filial filial) {
		this.filiais.add(filial);
	}

	/**
	 * Sobrescrita do método toString() para exibir as informações da empresa.
	 * 
	 * @return uma string com as informações da empresa
	 */
	@Override
	public String toString() {
		return "Empresa [nome=" + nome + ", endereço=" + endereço + ", cnpj=" + cnpj + ", filiais=" + filiais
				+ ", getNome()=" + getNome() + ", getEndereço()=" + getEndereço() + ", getCnpj()=" + getCnpj()
				+ ", getFiliais()=" + getFiliais() + ", getClass()=" + getClass() + ", hashCode()=" + hashCode()
				+ ", toString()=" + super.toString() + "]";
	}
}
